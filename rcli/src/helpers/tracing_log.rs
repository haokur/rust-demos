use crate::configs::consts::IS_PRODUCTION;
use crate::utils::text;
use log::Record;
use std::io;
use tracing::{Level, debug, info, span, subscriber};
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry, fmt};
// tracing 记录事件/跨度。
// tracing-subscriber 决定如何收集和处理这些数据（如输出到控制台）。
// tracing-log 可选地将传统 log 日志转发到 tracing。
// tracing-test 在测试中验证日志行为。
// tracing-appender：专注日志的异步写入和文件管理

pub fn init_logger() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(tracing_appender::rolling::Rotation::DAILY) // 每日滚动
        .filename_prefix("cli")
        .filename_suffix("log")
        .build("logs")
        .expect("Failed to create appender");

    let (non_blocking, guard) = non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_timer(ChronoLocal::default())
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_line_number(false)
        .with_file(false)
        .with_target(false)
        .with_filter(EnvFilter::from("info"));

    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_timer(ChronoLocal::default())
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_file(true)
        .with_filter(EnvFilter::from("debug"));

    Registry::default()
        .with(file_layer)
        .with((!*IS_PRODUCTION).then_some(stderr_layer))
        .init();

    guard
}

#[test]
fn test_tracing() {
    let _ = init_logger();
    process_order(123123);
}

fn validate_order() {
    let span = span!(Level::DEBUG, "validate order");
    let _enter = span.enter();
    debug!("validate order");
}

fn process_order(order_id: u64) {
    let span = span!(Level::INFO, "process_order", order_id);
    let _enter = span.enter();

    info!("Processing order");
    validate_order();
    info!("Order processing complete");
}
