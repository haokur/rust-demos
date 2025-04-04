use clap::{Parser, Subcommand};
use crate::application::cli::commands;

#[derive(Parser)]
#[command(name = "rcli")]
#[command(version = "1.0.0")]
#[command(about="another cli but rust",long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "say hello to who with --name params")]
    Hello {
        #[arg(short, long)]
        name: String,
    },

    #[command(about = "test inquire usage")]
    Inquire {},
}

pub fn dispatch_command() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Hello { name } => {
            commands::hello::run(&name);
        }
        Commands::Inquire {} => {
            commands::inquire_test::run();
        }
    }
}

