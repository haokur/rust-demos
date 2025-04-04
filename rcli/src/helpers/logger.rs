use log::LevelFilter;
use simplelog::{Config, WriteLogger};
use std::fs::File;

pub fn init_log() {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("my_log.txt").expect("log file can't be created"),
    )
        .expect("logger init failed");
}
