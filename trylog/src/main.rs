use log::{error, info, warn};
use log4rs;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    info!("booting up");
    warn!("test warn log");
    error!("test error log");

    info!(target: "requests", "test only");
    warn!(target: "requests", "test only");
    error!(target: "requests", "test only")
}
