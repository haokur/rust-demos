use crate::configs::consts::IS_PRODUCTION;
use crate::utils::text;
use flexi_logger::writers::LogWriter;
use flexi_logger::{
    Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, LoggerHandle, Naming,
};
use log::{Record, debug, info};
use std::sync::{Mutex, OnceLock};

static LOGGER_HANDLE: OnceLock<Mutex<LoggerHandle>> = OnceLock::new();

// 自定义写入
struct CustomWriter;
impl LogWriter for CustomWriter {
    fn write(&self, _now: &mut DeferredNow, _record: &Record) -> std::io::Result<()> {
        // println!("custom write record is {:?}", record);
        // 自定日志输出拦截，可推送到服务器，elasticsearch等
        Ok(())
    }
    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }
}

// 日志信息脱敏
fn sanitize_log_record(record: &Record) -> String {
    let record_str = record.args().to_string();
    text::desensitization(&record_str)
}

pub fn init_logger() {
    let format = |write: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record| {
        let time_str = now.format("%Y-%m-%d %H:%M:%S");
        write!(
            write,
            "{} [{}] {}",
            time_str,
            record.level(),
            sanitize_log_record(record)
        )
        .unwrap();
        Ok(())
    };

    let log_file_path = FileSpec::default()
        .directory("logs")
        .basename("cli")
        .suffix("log");
    // .discriminant(Local::now().format("%Y-%m-%d").to_string());

    let log_level = if *IS_PRODUCTION { "info" } else { "debug" };
    let duplicate_level = if *IS_PRODUCTION {
        Duplicate::Info
    } else {
        Duplicate::Debug
    };

    let logger_handle = Logger::try_with_str(log_level)
        .unwrap()
        .duplicate_to_stdout(duplicate_level)
        .format(format)
        .log_to_file_and_writer(log_file_path, Box::new(CustomWriter))
        .rotate(
            Criterion::Size(10 * 1024 * 1024),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(10),
        )
        .append()
        .start()
        .unwrap();

    LOGGER_HANDLE
        .set(Mutex::new(logger_handle))
        .unwrap_or_else(|_| println!("LOGGER_HANDLE set logger_handle error"));
    info!("logger serve init successful");
}

#[allow(dead_code)]
pub fn get_logger_handle() -> &'static Mutex<LoggerHandle> {
    debug!("get_logger_handle running");
    LOGGER_HANDLE.get().expect("LOGGER_HANDLE not set")
}

#[test]
fn test_flexi_logger() {
    init_logger();
    info!("user phone number is 13312341234,ip address is 127.0.0.1");
    // let test_1k_data: String = std::iter::repeat("x").take(1024).collect();
    // info!("test_1k_data: {}", test_1k_data);
}

#[test]
fn test_toggle_log_level() {
    init_logger();
    debug!("test_toggle_log_level this will not be seen,because default logger level is info");
    let mut logger_handle = get_logger_handle().lock().unwrap();
    logger_handle.parse_and_push_temp_spec("debug").unwrap();
    debug!("test_toggle_log_level after parse_and_push_temp_spec level to debug");
}
