use lazy_static::lazy_static;
use log::{trace, LevelFilter};
use rustupolis::engine::core::Engine;
use rustupolis::engine::keybinds::{KeyBindListener, Tty};
use rustupolis::engine::layout::{get_layout, Layout, LAYOUT};
use rustupolis::logging::RemoteLoggerClient;
use rustupolis::terminal::screen::CleanScreen;
use std::io::stdout;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use rustupolis::city_layout::building_management::add_bulding;
use rustupolis::city_layout::save_manage::load_savegame;
use rustupolis::engine;
use rustupolis::engine::viewport::Viewport;



lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();
}

lazy_static! {
    pub static ref STDOUT: Tty = MouseTerminal::from(stdout().into_raw_mode().unwrap());
}

lazy_static! {
    pub static ref ENGINE: Mutex<Engine> = Mutex::new(Engine::new(Viewport::from_ratio(0.75, 1.0), STDOUT.deref()));
}

fn main() {
    log::set_logger(LOGGER.deref())
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();

    let _ = Layout::load_default_layout();
    let mut layout = get_layout();
    dbg!("heere");

    let mut engine = ENGINE.lock().unwrap();


    let buildings_drawables = layout.buildings.clone();
    let roads_drawables = layout.roads.clone();
    dbg!("heere3");

    drop(layout);

    let _clear = CleanScreen::new();



    for drawable in buildings_drawables {
        engine.register_drawable(Box::new(drawable));
    }
    for drawable in roads_drawables {
        engine.register_drawable(Box::new(drawable))
    }
    dbg!("heere2");




    engine.refresh();

    let e = Arc::new(Mutex::new(engine));

    let tosend = e.clone();

    let kb = KeyBindListener::new(tosend, STDOUT.deref());

    let _ = kb.thread.join();
}