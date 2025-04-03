use clap::{Parser, Subcommand};
use inquire::validator::Validation;
use inquire::{Autocomplete, CustomUserError, MultiSelect, Password, Select, Text};
use log::info;
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

#[test]
fn test_is_subsequence() {
    assert_eq!(is_subsequence("azb1c66", "abc"), true);
}
fn is_subsequence(haystack: &str, needle: &str) -> bool {
    if haystack.is_empty() {
        return true;
    }
    let mut needle_iter = needle.chars();
    let mut haystack_iter = haystack.chars();

    // 逐个字符地检查 needle 是否能按顺序出现在 haystack 中
    let match_result = needle_iter.all(|ch| haystack_iter.any(|c| c == ch));
    info!(
        "{}",
        format!("short is {haystack},long is {needle},match_result is {match_result}")
    );
    match_result
}

#[test]
fn test_highlight_subsequence() {
    println!("{}", highlight_subsequence("azb1c66", "abc"))
}
// 高亮显示匹配的子序列部分
fn highlight_subsequence(haystack: &str, needle: &str) -> String {
    let mut result = String::new();
    let mut needle_iter = needle.chars();
    for ch in haystack.chars() {
        if needle_iter.clone().any(|needle_ch| needle_ch == ch) {
            // 对匹配的字符进行高亮显示
            result.push_str(&format!("\x1b[4m\x1b[31m{}\x1b[0m\x1b[0m", ch)); // 红色高亮
            needle_iter.next(); // 移动 needle 中的字符指针
        } else {
            result.push(ch);
        }
    }
    result
}

fn remove_ansi_escape_codes(s: &str) -> String {
    let ansi_escape_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    ansi_escape_regex.replace_all(s, "").to_string()
}

#[test]
fn test_remove_ansi_escape_codes() {
    let ansi_str = highlight_subsequence("azb1c66", "abc");
    println!("{}", remove_ansi_escape_codes(&ansi_str));
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

            println!(
                "email is {}, password is {},gender is {},favorite fruits is {:?}",
                email, password, user_gender, favorite_fruits
            );
        }
    }
}
