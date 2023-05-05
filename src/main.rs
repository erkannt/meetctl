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
    /// Starts Meet in a Chrom[e|ium] instance with debug access
    Launch { profile: Option<String> },
    /// Takes a room name, alias or url
    Join { room: String },
    /// Starts sharing your screen
    Share {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Launch { profile }) => {
            let mut args = vec![
                "--remote-debugging-port=9222",
                "--no-first-run",
                "--no-default-browser-check",
            ];

            let parg: String;
            if let Some(p) = profile {
                parg = format!("--profile-directory={}", p);
                args.push(&parg)
            }

            args.push("https://meet.google.com");

            std::process::Command::new("chromium")
                .args(args)
                .spawn()
                .expect("Failed to launch");
        }
        Some(Commands::Join { room }) => {
            let debug_ws_url =
                backoff::retry(backoff::ExponentialBackoff::default(), get_debug_url)
                    .expect("Failed to get browser info");

            println!("{}", debug_ws_url);
            let browser = Browser::connect(debug_ws_url).expect("Failed to connect to browser");
            let tab = browser
                .get_tabs()
                .lock()
                .unwrap()
                .clone()
                .into_iter()
                .find(|t| t.get_title().expect("Can't get tab title").contains("Meet"))
                .unwrap();
            tab.activate().unwrap();
            tab.navigate_to("https://meet.google.com").unwrap();
            tab.wait_for_element("#i8").unwrap().focus().unwrap();

            tab.send_character(room)
                .unwrap()
                .press_key("Tab")
                .unwrap()
                .press_key("Enter")
                .unwrap();
        }
        Some(Commands::Share {}) => {
            println!("Not implemented")
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
