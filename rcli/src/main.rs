use log::info;

mod application;
mod helpers;
mod utils;

fn main() {
    helpers::logger::init_logger();
    // helpers::logger::init_log();
    info!("starting up");
    // utils::process::with_ctrl_c_handler(
    //     || {
    //         helpers::logger::init_log();
    //         application::cli::dispatch_command();
    //     },
    //     Some("user interrupt operation"),
    // );
}
