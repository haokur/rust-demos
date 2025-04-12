#[macro_export]
macro_rules! safe_info {
    ($($arg:tt)*)  => {
        {
            let msg = format!($($arg)*);
            let safe_msg = crate::utils::text::desensitization(&msg);
            tracing::info!("{}",safe_msg);
        }
    };
}
