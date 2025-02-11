use crate::logging::RemoteLoggerClient;
use lazy_static::lazy_static;
use log::LevelFilter;
use std::ops::Deref;

mod logging;

lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();
}

fn main() {
    log::set_logger(LOGGER.deref()).map(|()| log::set_max_level(LevelFilter::Trace)).unwrap();
    log::info!("Starting up...");
    log::error!("This is an error message");
    log::warn!("This is a warning");
    log::trace!("This is a debug message");
    log::debug!("This is a trace message");
}
