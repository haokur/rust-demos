use chrono::Local;
use std::fs::File;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{Level, debug, error, event, info, info_span, instrument, span, warn};
use tracing_log::LogTracer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Layer, Registry, fmt};
use tracing_test::traced_test;

#[test]
fn test_tracing() {
    fmt().with_max_level(LevelFilter::DEBUG).init();
    info!("hello world");
    warn!("hello world");
    error!("hello world");
    debug!("hello world");

    // 结构化日志
    let user_id = 42;
    let action = "login";
    info!(user_id, action, "user performed action");

    event!(Level::INFO, user_id, action, "user performed action");
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
#[test]
fn test_tracing_span() {
    fmt().with_max_level(LevelFilter::DEBUG).init();
    process_order(123456789);
}

#[test]
fn test_tracing_custom_print() {
    fmt()
        .with_span_events(FmtSpan::CLOSE) // 记录span的开始和结束
        .with_target(false) // 不显示目标模块
        .compact()
        .init();

    process_order(123456789);

    info!("hello world");
}

#[test]
fn test_print_to_file() {
    let log_file = File::create("logs/cli.log").expect("create failed");

    // 文件日志无颜色
    let file_layer = fmt::layer().with_writer(log_file).with_ansi(false);

    // 中断日志带颜色
    let stderr_layer = fmt::layer().with_writer(std::io::stderr).with_ansi(true);

    let subscriber = Registry::default().with(file_layer).with(stderr_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!("hello world");
}

#[instrument]
async fn async_task(task_id: u64) {
    info!("Starting async task id: {}", task_id);
    sleep(Duration::from_secs(1)).await;
    info!("Finished async task id: {}", task_id);
}

#[tokio::test]
async fn test_async_logging() {
    fmt().with_max_level(LevelFilter::DEBUG).init();
    async_task(42).await;
}

// 性能考虑
#[test]
fn high_performance_task() {
    let subscriber = fmt()
        .without_time()
        .with_target(false)
        .with_ansi(false)
        .finish();

    tracing::dispatcher::set_global_default(subscriber.into()).unwrap();
    let span = info_span!("high performance task");
    let _enter = span.enter();

    info!("high performance task");
}

#[test]
fn test_custom_diy() {
    // 创建自定义过滤器
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .unwrap();

    // 创建自定义格式层
    let fmt_layer = fmt::layer().with_target(false).compact();

    // 组合订阅者
    let subscriber = Registry::default().with(filter).with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("hello world");
}

#[test]
fn test_custom_time() {
    fmt()
        .with_max_level(LevelFilter::DEBUG)
        .with_timer(fmt::time::ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
        .init();
    tracing::info!("hello world");
}

// tracing_subscriber::fmt()
//     .with_max_level(Level::DEBUG)
//     .init();