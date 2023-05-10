use clap::{Parser, Subcommand};
use headless_chrome::Browser;
use serde::Deserialize;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts Meet in a chromium instance with debug access
    Launch { profile: String },
    /// Takes a room name, alias or url
    Join { room: String },
    /// Share you entire screen
    Share,
    /// Create a new meeting and output its url
    New,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Launch { profile }) => {
            let options = run_script::ScriptOptions::new();
            run_script::spawn_script!(
                r#"
                 chromium \
                 --remote-debugging-port=9222 \
                 --no-first-run \
                 --no-default-browser-check \
                 --auto-select-desktop-capture-source="Entire screen" \
                 $1 \
                 https://meet.google.com &
                "#,
                &vec![format!("--profile-directory={}", profile)],
                &options
            )
            .unwrap();
        }
        Some(Commands::Join { room }) => {
            let browser = connect_to_browser();

            let mut room_url = room.to_string();
            if !room.contains("meet.google.com") {
                room_url = format!("https://meet.google.com/{}", room);
            }

            join_room(&browser, room_url);
            close_empty_tabs(browser);
        }
        Some(Commands::Share) => {
            let browser = connect_to_browser();
            let tab = get_meet_tab(&browser);
            tab.activate().unwrap();

            tab.find_element("button[aria-label='Present now']")
                .unwrap()
                .focus()
                .unwrap();
            tab.press_key("Enter").unwrap().press_key("Enter").unwrap();

            close_empty_tabs(browser);
        }
        Some(Commands::New) => {
            let browser = connect_to_browser();
            let tab = get_meet_tab(&browser);
            tab.activate().unwrap();

            let meeting_url = tab
                .navigate_to("https://meet.google.com/new")
                .unwrap()
                .wait_until_navigated()
                .unwrap()
                .get_url();
            println!("{}", meeting_url);

            close_empty_tabs(browser);
        }
        None => {}
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct VersionResponse {
    webSocketDebuggerUrl: String,
}

fn get_debug_url() -> Result<String, backoff::Error<reqwest::Error>> {
    let response: VersionResponse =
        reqwest::blocking::get("http://127.0.0.1:9222/json/version")?.json()?;
    return Ok(response.webSocketDebuggerUrl);
}

fn connect_to_browser() -> headless_chrome::Browser {
    let debug_ws_url = backoff::retry(backoff::ExponentialBackoff::default(), get_debug_url)
        .expect("Failed to get debug url");
    return Browser::connect(debug_ws_url).expect("Failed to connect to browser");
}

fn get_meet_tab(browser: &Browser) -> std::sync::Arc<headless_chrome::Tab> {
    browser
        .get_tabs()
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|t| t.get_url().contains("meet.google.com"))
        .unwrap()
}

fn join_room(browser: &Browser, room_url: String) {
    let tab = browser
        .get_tabs()
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|t| t.get_url().contains("meet.google.com"))
        .unwrap();
    tab.activate().unwrap();
    tab.navigate_to(&room_url).unwrap();
}

fn close_empty_tabs(browser: Browser) {
    browser
        .get_tabs()
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|t| t.get_url().contains("about:blank"))
        .map(|t| t.close(false).unwrap())
        .for_each(drop);
}
