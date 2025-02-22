use lazy_static::lazy_static;
use log::{trace, LevelFilter};
use rustupolis::engine::core::Engine;
use rustupolis::engine::keybinds::{KeyBindListener, Tty};
use rustupolis::logging::RemoteLoggerClient;
use rustupolis::terminal::screen::CleanScreen;
use std::io::stdout;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use rustupolis::engine::viewport::Viewport;

lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();
}

lazy_static! {
    pub static ref STDOUT: Tty = MouseTerminal::from(stdout().into_raw_mode().unwrap());
}

fn main() {
    log::set_logger(LOGGER.deref())
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();

    let layout = rustupolis::engine::layout::read_layout();

    let buildings = layout.buildings;
    let buildings_drawables = rustupolis::engine::layout::drawables_from_buildings(buildings);

    let roads = layout.roads;
    let roads_drawables = rustupolis::engine::layout::drawables_from_roads(roads);


    let _clear = CleanScreen::new();

    let mut vp = Viewport::default();

    vp.width = (terminal_size().unwrap().0 as f32 * 0.75) as u16;

    trace!("viewport: {:?}", vp);

    let mut engine = Engine::new(vp, STDOUT.deref());

    for drawable in buildings_drawables {
        engine.register_drawable(Box::new(drawable))
    }
    for drawable in roads_drawables {
        engine.register_drawable(Box::new(drawable))
    }

    engine.refresh();

    let e = Arc::new(Mutex::new(engine));

    let tosend = e.clone();

    let kb = KeyBindListener::new(tosend, STDOUT.deref());

    let _ = kb.thread.join();
}