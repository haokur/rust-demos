use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rcli")]
#[command(version = "1.0.0")]
#[command(about="another cli but rust",long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "say hello to who with --name params")]
    Hello {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello { name } => {
            println!("Hello, {}!", name);
        }
    }
}
