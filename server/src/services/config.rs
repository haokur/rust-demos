use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: Server,
    pub database: Database,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub mysql_host: String,
    pub mysql_port: String,
    pub mysql_user: String,
    pub mysql_password: String,
    pub mysql_database: String,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        // 读取环境变量 APP_ENV，默认是 prod
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "prod".to_string());
        let path = format!("configs/{}.yml", env);

        // 加载配置
        let settings = config::Config::builder()
            .add_source(config::File::with_name(&path))
            .build()
            .unwrap();

        // 反序列化配置
        settings.try_deserialize().unwrap()
    };
}
