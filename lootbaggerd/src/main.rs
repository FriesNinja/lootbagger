use core::core;

use log::{info};
use simple_logger::SimpleLogger;


fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Hello, world!");

    core();
}
