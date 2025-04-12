use regex::Regex;
use std::sync::OnceLock;

#[test]
fn test_ip_regex() {
    let str = "127.0.0.1";
    let result = ip_regex_desensitization().replace_all(&str, "$1.***.***.$4");
    println!("{}", result);
}

#[allow(dead_code)]
pub fn ip_regx() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").unwrap())
}

pub fn ip_regex_desensitization() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})").unwrap())
}

#[test]
fn test_phone_regex() {
    let str = "13312341234";
    println!("{:?}", phone_regex().find(str));
    let result = phone_regex().replace_all(str, "****");

    println!("result is {}", result);

    let result = phone_regex_desensitization().replace_all(&str, "$1****$2");
    println!("desensitization phone result is {}", result);
}

#[allow(dead_code)]
pub fn phone_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    &REGEX.get_or_init(|| Regex::new(r"\b1[3-9]\d{9}\b").unwrap())
}

pub fn phone_regex_desensitization() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"(\d{3})\d{4}(\d{4})").unwrap())
}
