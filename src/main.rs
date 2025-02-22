use crate::logging::RemoteLoggerClient;
use lazy_static::lazy_static;
use log::LevelFilter;
use rustupolis::engine::keybinds::{KeyBindListener, Tty};
use rustupolis::engine::layout;
use rustupolis::terminal::screen::CleanScreen;
use rustupolis::ui::sidebar::SideBar;
use std::io::stdout;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use rustupolis::engine::core::Engine;
use rustupolis::engine::viewport::Viewport;

mod logging;

lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();
    pub static ref STDOUT: Tty = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    pub static ref SIDE_BAR: Arc<Mutex<SideBar>> = Arc::new(Mutex::new(SideBar::new()));
}

fn main() {
    log::set_logger(LOGGER.deref())
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();

    let _clear = CleanScreen::new();

    let layout = layout::read_layout();

    let bdrawables = layout::drawables_from_buildings(layout.buildings);
    let rdrawables = layout::drawables_from_roads(layout.roads);

    let mut vp = Viewport::default();

    vp.width = (terminal_size().unwrap().0 as f32 * 0.75) as u16;

    let mut engine = Engine::new(vp, STDOUT.deref());

    for d in bdrawables {
        engine.register_drawable(Box::new(d));
    }

    for d in rdrawables {
        engine.register_drawable(Box::new(d));
    }

    engine.refresh();

    let e = Arc::new(Mutex::new(engine));

    let kb_engine_ref = e.clone();

    let kb = KeyBindListener::new(kb_engine_ref, STDOUT.deref());

    let _ = kb.thread.join();
}
