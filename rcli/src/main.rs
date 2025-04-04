mod application;
mod helpers;
mod utils;

fn main() {
    utils::process::with_ctrl_c_handler(
        || {
            helpers::logger::init_log();
            application::cli::dispatch_command();
        },
        Some("user interrupt operation"),
    );
}
