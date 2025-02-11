use ansi_term::Color;
use lazy_static::lazy_static;
use log::LevelFilter;
use rustupolis::engine::core::Engine;
use rustupolis::engine::drawable::DynDrawable;
use rustupolis::engine::keybinds::KeyBindListener;
use rustupolis::engine::test::{Test2Drawable, TestDrawable, QG};
use rustupolis::logging::RemoteLoggerClient;
use rustupolis::terminal::screen::CleanScreen;
use std::io::stdout;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use termion::color::{Blue, Green, LightBlack, Red};
use termion::raw::IntoRawMode;

lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();
}

fn main() {
    log::set_logger(LOGGER.deref())
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();

    let s = stdout().into_raw_mode().unwrap();

    let _clear = CleanScreen::new();

    let mut engine = Engine::default();

    let test = TestDrawable {
        xpos: 10,
        ypos: 25,
        color: Color::Green,
    };
    let test2 = Test2Drawable {
        xpos: 2,
        ypos: 2,
        color: Color::Blue,
    };
    let test3 = Test2Drawable {
        xpos: 2,
        ypos: 5,
        color: Color::Green,
    };
    let qg = QG {
        xpos: 10,
        ypos: 15,
        color: Color::Red,
    };

    let t: Box<DynDrawable> = Box::new(test);
    let t2: Box<DynDrawable> = Box::new(test2);
    let t3: Box<DynDrawable> = Box::new(test3);
    let tqg: Box<DynDrawable> = Box::new(qg);

    engine.register_drawable(t);
    engine.register_drawable(t2);
    engine.register_drawable(t3);
    engine.register_drawable(tqg);

    engine.refresh();

    let e = Arc::new(Mutex::new(engine));

    let tosend = e.clone();

    let kb = KeyBindListener::new(tosend);

    let _ = kb.thread.join();
}
