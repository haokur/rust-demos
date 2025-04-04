mod cli;
mod logger;
mod utils;
mod commands;

fn main() {
    utils::with_ctrl_c_handler(
        || {
            logger::init_log();
            cli::dispatch_command();
        },
        Some("user interrupt operation"),
    );
}
