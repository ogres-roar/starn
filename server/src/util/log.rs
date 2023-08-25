/// log
use log::warn;
use std::default::Default;

pub fn init_logger() {
    match log4rs::init_file("conf/log.toml", Default::default()) {
        Err(e) => warn!(target:"starn", "parse conf/log.toml fail: {}", e),
        _ => (),
    }
}
