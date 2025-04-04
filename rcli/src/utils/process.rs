use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn with_ctrl_c_handler<F: FnOnce()>(main_logic: F, exit_message: Option<&str>) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let msg = exit_message.unwrap_or("progress is exit").to_string();
    ctrlc::set_handler(move || {
        if r.load(Ordering::SeqCst) {
            println!("\n {} Caught Ctrl-C, shutting down.", msg);
            std::process::exit(0);
        }
    })
    .expect("Error setting Ctrl-C handler");
    main_logic();
}
