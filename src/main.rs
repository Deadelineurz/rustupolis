use crate::logging::RemoteLoggerClient;
use lazy_static::lazy_static;
use log::{debug, LevelFilter};
use rustupolis::engine::core::Engine;
use rustupolis::engine::keybinds::KeyBindListener;
use rustupolis::engine::layout::{Layout, LayoutId};
use rustupolis::engine::viewport::Viewport;
use rustupolis::roads::road_graph::Graph;
use rustupolis::terminal::screen::CleanScreen;
use rustupolis::threads::demo::demo_scope;
use rustupolis::threads::engine_loop::engine_loop;
use rustupolis::threads::sidebar::sidebar;
use rustupolis::ui::sidebar::{LogColor, LogType};
use std::io::stdout;
use std::ops::Deref;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use rustupolis::threads::sidebar::SideBarMessage::Quit;
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

    let mut wonder_graph = Graph::new(&layout);
    wonder_graph.start_dfs(&layout);

    let stdout = Arc::from(MouseTerminal::from(stdout().into_raw_mode().unwrap()));

    // ----- UI SETUP -----
    let mut vp = Viewport::default();
    let (ter_x, ter_y) = terminal_size().unwrap();

    vp.width = (terminal_size().unwrap().0 as f32 * 0.75) as u16 - 1;

    vp.output_y = (ter_y as f32 * 0.1) as u16 + 2;
    vp.height = ter_y - vp.output_y;

    let (sidebar_chan, sidebar) = sidebar(stdout.clone());

    let mut engine = Engine::new(vp, stdout.clone(), sidebar_chan.clone(), layout);


    engine.refresh();

    let e = Arc::new(RwLock::new(engine));

    thread::scope(|s| {
        let (click_sender, click_receiver) = channel();
        let (key_sender, key_receiver) = channel();
        let kb = KeyBindListener::new(s, e.clone(), vec![click_sender], vec![key_sender], sidebar_chan.clone());
        let demo = demo_scope(s, e.clone(), kb.stop_var.clone());
        let game_loop = engine_loop(s, e.clone(), kb.stop_var.clone(), click_receiver, key_receiver);
        let _ = kb.thread.join();
        let _ = demo.join();
        let _ = game_loop.join();
        let _ = sidebar_chan.send(Quit);
        let _ = sidebar.join();
    });

    println!("{}", Arc::strong_count(&stdout));
    drop(e);
}
