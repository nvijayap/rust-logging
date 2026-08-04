use log::{debug, error, info, trace, warn};

fn main() {

    // For env_logger, set RUST_LOG env var to one of the log levels
    env_logger::init();

    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is a info message");
    warn!("This is a warn message");
    error!("This is a error message");
}
