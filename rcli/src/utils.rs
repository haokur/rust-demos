use log::info;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[test]
fn test_is_subsequence() {
    assert_eq!(is_subsequence("azb1c66", "abc"), true);
}
pub fn is_subsequence(haystack: &str, needle: &str) -> bool {
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
pub fn highlight_subsequence(haystack: &str, needle: &str) -> String {
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

#[test]
fn test_remove_ansi_escape_codes() {
    let ansi_str = highlight_subsequence("azb1c66", "abc");
    println!("{}", remove_ansi_escape_codes(&ansi_str));
}

pub fn remove_ansi_escape_codes(s: &str) -> String {
    let ansi_escape_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    ansi_escape_regex.replace_all(s, "").to_string()
}

pub fn with_ctrl_c_handler<F: FnOnce()>(main_logic: F, exit_message: Option<&str>) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let msg = exit_message.unwrap_or("progress is exit").to_string();
    ctrlc::set_handler(move || {
        if r.load(Ordering::SeqCst) {
            println!("\n {} Caught Ctrl-C, shutting down.", msg);
            std::process::exit(0);
        }
    })
    .expect("Error setting Ctrl-C handler");
    main_logic();
}
