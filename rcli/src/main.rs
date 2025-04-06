mod application;
mod configs;
mod helpers;
mod macros;
mod utils;

fn main() {
    utils::process::with_ctrl_c_handler(
        || {
            let _guard = helpers::tracing_log::init_logger();
            application::cli::dispatch_command();
        },
        Some("user interrupt operation"),
    );
}
