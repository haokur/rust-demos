use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref IS_PRODUCTION: bool = {
        // 环境变量优先，否则根据编译模式决定
        env::var("PRODUCTION").map(|v| v == "true").unwrap_or_else(|_| {
            !cfg!(debug_assertions) // 发布模式时为 true
        })
    };
}
