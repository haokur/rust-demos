use chrono::Local;
use flexi_logger::{Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, Naming};
use log::{LevelFilter, Record, debug, error, info};
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
fn test_simple_log() {
    init_log();
    info!("test_log");
    debug!("debug log content");
    error!("error log content");
}


pub fn init_logger() {
    let format = |write: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record| {
        let time_str = now.format("%Y-%m-%d %H:%M:%S");
        write!(write, "{} [{}] {}", time_str, record.level(), record.args()).unwrap();
        Ok(())
    };

    let log_file_path = FileSpec::default()
        .directory("logs")
        .basename("cli")
        .suffix("log");
    // .discriminant(Local::now().format("%Y-%m-%d").to_string());

    Logger::try_with_str("info")
        .unwrap()
        .duplicate_to_stdout(Duplicate::All)
        .format(format)
        .log_to_file(log_file_path)
        .rotate(
            Criterion::Size(10 * 1024 * 1024),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(10),
        )
        .append()
        .start()
        .unwrap();
}

#[test]
fn test_flexi_logger() {
    init_logger();
    // info!("test_log");
    // debug!("debug log content");
    // error!("error log content");
    let test_1k_data: String = std::iter::repeat("x").take(1024).collect();
    info!("test_1k_data: {}", test_1k_data);
}
