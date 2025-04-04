use crate::application::cli::commands;
use clap::{Parser, Subcommand};

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
    Hello { name: String },

    #[command(about = "test inquire usage")]
    Inquire {},

    #[command(about = "kill progress by port or program name")]
    Kill { port_or_program_name: String },
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
        Commands::Kill {
            port_or_program_name,
        } => {
            let name_or_ports: Vec<&str> = port_or_program_name.split(',').collect();
            commands::kill::run(name_or_ports);
        }
    }
}
