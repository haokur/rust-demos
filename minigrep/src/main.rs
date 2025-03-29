use minigrep::{Config, run};
use std::{env, process};

// 获取命令行参数 query 和 file_path
// 通过file_path读取内容，使用query参数去逐行匹配，获得一个匹配的结果
// 输出这个匹配的结果
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
