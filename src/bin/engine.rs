use lazy_static::lazy_static;
use log::LevelFilter;
use rustupolis::engine::core::Engine;
use rustupolis::engine::drawable::Drawable;
use rustupolis::engine::test::TestDrawable;
use rustupolis::logging::RemoteLoggerClient;
use rustupolis::terminal::screen::CleanScreen;
use std::io::{stdout, Write};
use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;

lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();
}

fn main() {
    log::set_logger(LOGGER.deref()).map(|()| log::set_max_level(LevelFilter::Trace)).unwrap();

    let _clear = CleanScreen::new();

    let mut engine = Engine::default();

    let test = TestDrawable{};

    let t: Box<dyn Drawable> = Box::new(test);

    engine.register_drawable(t);

    engine.refresh();
    
    stdout().flush().expect("TODO: panic message");

    sleep(Duration::from_secs(5))
}