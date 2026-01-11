mod commands;
mod config;
mod engine;
mod storage;
mod tui;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "flow")]
#[command(about = "A CLI for managing and piping prompts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new prompt
    Add {
        /// Alias for the prompt
        #[arg(short, long)]
        alias: Option<String>,
        /// Tags for the prompt (comma separated)
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// List all prompts
    #[command(name = "ls", visible_alias = "list")]
    List,
    /// Use a prompt
    Use {
        /// The alias of the prompt to use
        alias: String,
        /// Print output to stdout instead of clipboard
        #[arg(short, long)]
        print: bool,
    },
    /// Search for a prompt
    Search,
    /// Launch the interactive TUI
    Ui,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { alias, tags } => {
            commands::add(alias.clone(), tags.clone());
        }
        Commands::List => {
            commands::list();
        }
        Commands::Use { alias, print } => {
            commands::use_prompt(alias.clone(), *print);
        }
        Commands::Search => {
            commands::search();
        }
        Commands::Ui => {
            commands::ui();
        }
    }
}
