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
    Launch {},
    /// Takes a room name, alias or url
    Join { room: String },
    /// Starts sharing your screen
    Share {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Launch {}) => {
            println!("Not implemented")
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
