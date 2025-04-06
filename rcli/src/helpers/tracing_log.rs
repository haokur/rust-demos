use crate::configs::consts::IS_PRODUCTION;
use crate::utils::text;
use tracing::{Event, Subscriber};
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::{
    FmtContext, FormatEvent, FormatFields, format::Writer, time::ChronoLocal,
};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry};

// tracing 记录事件/跨度。
// tracing-appender：专注日志的异步写入和文件管理
// tracing-subscriber 决定如何收集和处理这些数据（如输出到控制台）。
// tracing-log 可选地将传统 log 日志转发到 tracing。
// tracing-test 在测试中验证日志行为。

pub struct RedactingFormatter;

// 谨慎使用，每个日志都要走脱敏格式化，损耗性能，直接在调用info!等时脱敏传入
// 或者使用safe_info!等
impl<S, N> FormatEvent<S, N> for RedactingFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let mut buf = String::new();
        tracing_subscriber::fmt::format().format_event(ctx, Writer::new(&mut buf), event)?;

        let redacted = text::desensitization(&buf);

        write!(writer, "{}", redacted)
    }
}

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
        .with_filter(EnvFilter::from("trace"));

    Registry::default()
        .with(file_layer)
        .with((!*IS_PRODUCTION).then_some(stderr_layer))
        .init();

    guard
}
