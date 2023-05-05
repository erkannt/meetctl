use std::{fmt::format, vec};

use clap::{Parser, Subcommand};

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

            std::process::Command::new("chromium")
                .args(args)
                .spawn()
                .expect("Failed to launch");

            let browser_info =
                backoff::retry(backoff::ExponentialBackoff::default(), get_browser_info)
                    .expect("Failed to get browser info");
            println!("{}", browser_info)
        }
        Some(Commands::Join { room }) => {
            println!("Not implemented ({})", room)
        }
        Some(Commands::Share {}) => {
            println!("Not implemented")
        }
        None => {}
    }
}

fn get_browser_info() -> Result<String, backoff::Error<reqwest::Error>> {
    let text = reqwest::blocking::get("http://127.0.0.1:9222/json/version")?.text()?;
    return Ok(text);
}
