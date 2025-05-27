use crate::logging::RemoteLoggerClient;
use lazy_static::lazy_static;
use log::{debug, LevelFilter};
use rustupolis::engine::core::Engine;
use rustupolis::engine::keybinds::{Clickable, KeyBindListener};
use rustupolis::engine::layout::{Layout, LayoutId, Road};
use rustupolis::engine::viewport::Viewport;
use rustupolis::roads::road_graph::{Graph, Rect};
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
use base64::Engine as EngineBase64;
use base64::prelude::BASE64_STANDARD;
use std::option::Option;
use std::thread::sleep;
use std::time::Duration;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use rustupolis::engine::drawable::Drawable;
use rustupolis::threads::sidebar::SideBarMessage::Quit;
mod logging;

lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();
}

fn intersection(r1: &dyn Drawable, r2: &dyn Drawable) -> Option<Rect> {
    let tolerance: i16 = 1;

    let x1 = r1.x().max(r2.x());
    let y1 = r1.y().max(r2.y());
    let x2 = (r1.x() + r1.width() as i16).min(r2.x() + r2.width() as i16);
    let y2 = (r1.y() + r1.height() as i16).min(r2.y() + r2.height() as i16);

    // Allow for tolerance of 1 pixel in both x and y directions
    if x1 < x2 + tolerance && y1 < y2 + tolerance {
        Some(Rect {
            x: x1,
            y: y1,
            width: ((x2 - x1).max(0) + tolerance) as u8, // ensure non-negative, add tolerance
            height: ((y2 - y1).max(0) + tolerance) as u8,
        })
    } else {
        None
    }
}


fn main() {
    log::set_logger(LOGGER.deref())
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .unwrap();

    let _clear = CleanScreen::new();

    let mut layout = Layout::load_default_layout();

    let mut wonder_graph = Graph::new(&layout);
    wonder_graph.start_dfs(&layout);

    //println!("{:?}",layout);

    let path = wonder_graph.find_path_bfs(&layout.buildings[0].id, &layout.buildings[18].id);

    let mut last_drawable : Option<Box<dyn Drawable>> = None;
    let mut intersections = vec![];
    if let Some(chemin) = path {
        println!("Itinéraire trouvé:");
        for id in chemin {
            for bldg in layout.buildings.iter().filter(|x| x.id == id){
                println!("{:?}", bldg.name);
                /*if last_drawable.is_some(){
                    println!("{:?}", intersection(&*last_drawable.unwrap(), &*Box::new(bldg.clone())))
                }*/
                if last_drawable.is_some() {
                    intersections.push(intersection(&*last_drawable.unwrap(), &*Box::new(bldg.clone())));
                }
                last_drawable = Some(Box::new(bldg.clone()))
            }
            for rdg in layout.roads.iter().filter(|x| x.id == id){
                println!("{:?}", rdg.name);

                /*if last_drawable.is_some(){
                    println!("{:?}", intersection(&*last_drawable.unwrap(), &*Box::new(rdg.clone())))
                }*/
                if last_drawable.is_some() {
                    intersections.push(intersection(&*last_drawable.unwrap(), &*Box::new(rdg.clone())));
                }
                last_drawable = Some(Box::new(rdg.clone()))
            }
        }
    } else {
        println!("Aucun itinéraire trouvé.");
    }

    let mut to_highlight = vec![];
    for (i, window) in intersections.windows(2).enumerate() {
        if let [Some(inter), Some(inter2)] = window {
            let hori = !(inter.x == inter2.x || (inter.x - inter2.x).abs() <= 3);
            println!("GPS{:} horiz : {:}", i, hori);
            to_highlight.push(Road {
                name: format!("GPS{}", i),
                id: LayoutId::random(),
                start_x: if inter.x > inter2.x {inter2.x} else { inter.x },
                start_y: if inter.y > inter2.y {inter2.y} else { inter.y },
                horizontal: if hori {true} else { false },
                width: if hori {inter.height -1 } else { inter.width },
                length: if hori {
                    (inter2.x - inter.x).abs() as u8
                } else {
                    (inter2.y - inter.y).abs() as u8
                },
                pavement: '░',
            });
        }
    }
    println!("{:?}", intersections);
    println!("{:?}", to_highlight);
    layout.roads.extend(to_highlight);

    //sleep(Duration::from_secs(2));

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
