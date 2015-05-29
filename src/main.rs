#[macro_use]
extern crate log;
extern crate env_logger;

mod uci;
use uci::UCI;

fn main() {
    env_logger::init().unwrap();
    info!("Starting up ...");
    UCI::start();
}
