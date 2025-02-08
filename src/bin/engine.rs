use lazy_static::lazy_static;
use log::LevelFilter;
use rustupolis::engine::core::Engine;
use rustupolis::engine::drawable::DynDrawable;
use rustupolis::engine::test::TestDrawable;
use rustupolis::logging::RemoteLoggerClient;
use rustupolis::terminal::screen::CleanScreen;
use std::io::stdout;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use termion::raw::IntoRawMode;
use rustupolis::engine::keybinds::KeyBindListener;

lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();
}

fn main() {
    log::set_logger(LOGGER.deref()).map(|()| log::set_max_level(LevelFilter::Trace)).unwrap();

    let s = stdout().into_raw_mode().unwrap();

    let _clear = CleanScreen::new();

    let mut engine = Engine::default();

    let test = TestDrawable{};

    let t: Box<DynDrawable> = Box::new(test);

    engine.register_drawable(t);

    engine.refresh();

    let e = Arc::new(Mutex::new(engine));

    let tosend = e.clone();

    let kb = KeyBindListener::new(tosend);

    let _ = kb.thread.join();
}