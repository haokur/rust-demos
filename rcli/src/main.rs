mod utils;
use crate::utils::*;

use clap::{Parser, Subcommand};
use inquire::validator::Validation;
use inquire::{Autocomplete, Confirm, CustomUserError, MultiSelect, Password, Select, Text};
use simplelog::*;

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

    #[command(about = "test inquire usage")]
    Inquire {},
}

#[derive(Debug, Clone)]
struct EmailAutoComplete;

impl Autocomplete for EmailAutoComplete {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        let emails = vec!["a@qq.com", "az@qq.com", "b@qq.com", "c@qq.com"];
        let suggestions = emails
            .into_iter()
            .filter(|email: &&str| is_subsequence(email, input))
            .map(|email| highlight_subsequence(email, input))
            .collect();

        Ok(suggestions)
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Option<String>, CustomUserError> {
        let suggestion = self.get_suggestions(input)?;
        if let Some(highlighted) = highlighted_suggestion {
            return Ok(Some(highlighted));
        }

        Ok(suggestion
            .get(0)
            .cloned()
            .map(|s| remove_ansi_escape_codes(&s)))
    }
}

fn init_log() {
    // 初始化 logger，将日志输出到文件 "my_log.txt"
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        std::fs::File::create("my_log.txt").unwrap(),
    )
    .expect("Could not create logger");
}

fn main() {
    with_ctrl_c_handler(
        || {
            init_log();
            let cli = Cli::parse();

            match &cli.command {
                Commands::Hello { name } => {
                    println!("Hello, {}!", name);
                }
                Commands::Inquire {} => {
                    let email = Text::new("Email:")
                        .with_default("test@qq.com")
                        .with_help_message("Enter a Email")
                        .with_placeholder("abc@qq.com")
                        .with_validator(|input: &str| {
                            if input.contains("@") {
                                Ok(Validation::Valid)
                            } else {
                                Ok(Validation::Invalid(
                                    "please enter right email address".into(),
                                ))
                            }
                        })
                        .with_formatter(&|input: &str| input.trim().to_string())
                        .with_autocomplete(EmailAutoComplete)
                        .prompt()
                        .expect("Please enter your username");

                    let user_gender = Select::new("Choose your gender", vec!["boy", "girl"])
                        .with_help_message("Use ↑↓ to navigate, Enter to select")
                        .with_formatter(&|f| format!(">>> {}", f))
                        .prompt()
                        .unwrap();

                    let favorite_fruits = MultiSelect::new(
                        "Choose your favorite fruits",
                        vec!["apple", "orange", "banana"],
                    )
                    .with_help_message("with key space to choose")
                    .prompt()
                    .unwrap();

                    let password = Password::new("Password:")
                        .prompt()
                        .expect("Please enter your password");

                    let confirmed = Confirm::new("Do you want to continue?")
                        .with_default(true)
                        .prompt()
                        .unwrap();

                    if confirmed {
                        println!(
                            "email is {}, password is {},gender is {},favorite fruits is {:?}",
                            email, password, user_gender, favorite_fruits
                        );
                    } else {
                        println!("user canceled");
                    }
                }
            };
        },
        Some("user interrupt operation"),
    );
}
