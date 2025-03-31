use env_logger::{Builder, Target};
use log::Level::Debug;
use log::{Level, debug, error, info, log, log_enabled, trace, warn};

pub fn shave_the_yak() {
    trace!("Commencing yak shaving");
    info!("Yak shaving");
    warn!("You've been yaking on my false positive team");
    error!("You've been yaking on my true positive team");
}

fn main() {
    // env_logger::init();

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    println!("Hello, world!");

    shave_the_yak();

    if log_enabled!(Debug) {
        debug!("You've been yaking on my false positive team");
    }

    if log_enabled!(target: "Global",Debug) {
        debug!("You've been yaking on my true positive team");
    }

    let data = (42, "forty-two");
    let private_data = "private";

    log!(Level::Error, "Received errors:{},{}", data.0, data.1);
    log!(target:"app_events",Level::Warn,"App warning:{},{},{}",data.0,data.1,private_data);
}
