use lazy_static::lazy_static;
use log::LevelFilter;
use rustupolis::engine::core::Engine;
use rustupolis::engine::drawable::DynDrawable;
use rustupolis::engine::keybinds::KeyBindListener;
use rustupolis::engine::test::{Test2Drawable, TestDrawable};
use rustupolis::logging::RemoteLoggerClient;
use rustupolis::terminal::screen::CleanScreen;
use std::io::{stdout, Stdout};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};

lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();
}

lazy_static! {
    pub static ref STDOUT: MouseTerminal<RawTerminal<Stdout>> = MouseTerminal::from(stdout().into_raw_mode().unwrap());
}

fn main() {
    log::set_logger(LOGGER.deref()).map(|()| log::set_max_level(LevelFilter::Trace)).unwrap();

    let _clear = CleanScreen::new();

    let mut engine = Engine::from(STDOUT.deref());

    let test = TestDrawable{};
    let test2 = Test2Drawable{xpos:2, ypos:2};
    let test3 = Test2Drawable{xpos:2, ypos:5};

    let t: Box<DynDrawable> = Box::new(test);
    let t2: Box<DynDrawable> = Box::new(test2);
    let t3: Box<DynDrawable> = Box::new(test3);

    engine.register_drawable(t);
    engine.register_drawable(t2);
    engine.register_drawable(t3);

    engine.refresh();

    let e = Arc::new(Mutex::new(engine));

    let tosend = e.clone();

    let kb = KeyBindListener::new(tosend, STDOUT.deref());

    let _ = kb.thread.join();
}