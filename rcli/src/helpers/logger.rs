use log::{LevelFilter, debug, error, info};
use simplelog::{Config, ConfigBuilder, LevelPadding, WriteLogger};
use std::fs::{File, OpenOptions};
use time::OffsetDateTime;
use time::macros::format_description;

#[test]
fn test_format_description() {
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    println!("{:?}", now);
    let formatted = now.format(&format).unwrap();
    println!("{}", formatted);
}

fn get_config() -> Config {
    // set_location_level 是否打印代码位置
    let config = ConfigBuilder::new()
        .set_time_format_custom(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .set_time_offset_to_local()
        .unwrap()
        // .set_location_level(LevelFilter::Error)
        .set_level_padding(LevelPadding::Right)
        .build();

    config
}

pub fn init_log() {
    let config = get_config();
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("my_log.txt")
        .unwrap();

    WriteLogger::init(LevelFilter::Debug, config, log_file).expect("logger init failed");
}

#[test]
fn test_log() {
    init_log();
    info!("test_log");
    debug!("debug log content");
    error!("error log content");
}
