use log::info;

mod application;
mod configs;
mod helpers;
mod utils;

fn main() {
    utils::process::with_ctrl_c_handler(
        || {
            let _guard = helpers::tracing_log::init_logger();
            info!("starting up");
            info!("user phone number is 13312341234,ip address is 127.0.0.1");
            application::cli::dispatch_command();
        },
        Some("user interrupt operation"),
    );
}
