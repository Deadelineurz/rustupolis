use crate::logging::RemoteLoggerClient;
use lazy_static::lazy_static;
use log::LevelFilter;
use rand::seq::SliceRandom;
use rustupolis::engine::core::Engine;
use rustupolis::engine::keybinds::KeyBindListener;
use rustupolis::engine::layout::Layout;
use rustupolis::engine::viewport::Viewport;
use rustupolis::terminal::screen::CleanScreen;
use rustupolis::ui::sidebar::{LogColor, LogType, SideBar};
use std::fmt::Display;
use std::io::stdout;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::thread;
use rand::rng;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use rustupolis::threads::demo::demo_scope;
use rustupolis::threads::sidebar::sidebar;

mod logging;

lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();
}

fn main() {
    log::set_logger(LOGGER.deref())
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .unwrap();

    let _clear = CleanScreen::new();

    let layout = Layout::load_default_layout();

    let bdrawables = layout.get_buildings();
    let rdrawables = layout.get_roads();

    let mut vp = Viewport::default();

    vp.width = (terminal_size().unwrap().0 as f32 * 0.75) as u16 - 1;

    let stdout = Arc::from(MouseTerminal::from(stdout().into_raw_mode().unwrap()));

    let (sidebar_chan, sidebar) = sidebar(stdout.clone());

    let mut engine = Engine::new(vp, stdout.clone(), sidebar_chan.clone());

    for d in bdrawables {
        engine.register_drawable(Box::new(d));
    }

    for d in rdrawables {
        engine.register_drawable(Box::new(d));
    }

    engine.refresh();

    let e = Arc::new(RwLock::new(engine));

    thread::scope(|s| {
        let kb = KeyBindListener::new(s, e.clone());
        let demo = demo_scope(s, e.clone(), kb.stop_var.clone());
        let _ = kb.thread.join();
        let _ = demo.join();
        let _ = sidebar_chan.send((vec![Box::new("")], LogType::Debug, LogColor::Normal));
        let _ = sidebar.join();
    });
}
