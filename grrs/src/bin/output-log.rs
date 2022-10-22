use log::{info, warn};

// env RUST_LOG=info cargo run --bin output-log
fn main() {
  env_logger::init();
  info!("starting up");
  warn!("oops, nothing implemented!");
}
