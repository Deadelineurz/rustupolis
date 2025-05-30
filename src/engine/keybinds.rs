use std::{env, fs};
use crate::engine::core::{Engine};
use crate::utils::interruptible_sleep::InterruptibleSleep;
use log::{debug, trace};
use std::io::{stdin, Stdout};
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::thread::{Scope, ScopedJoinHandle};
use termion::cursor;
use termion::event::{Event, Key, MouseButton, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::RawTerminal;
use crate::threads::sidebar::SideBarMessage;
use crate::ui::sidebar::SyncDisplay;
use crate::utils::{send_to_side_bar_write};

pub type Tty = MouseTerminal<RawTerminal<Stdout>>;

pub trait Clickable {
    fn infos(&self, engine: &Engine) -> Option<Vec<String>> {
        None
    }
}

pub struct KeyBindListener<'scope> {
    pub thread: ScopedJoinHandle<'scope, ()>,
    pub stop_var: Arc<InterruptibleSleep>
}

pub static RUNNING: AtomicBool = AtomicBool::new(true);

impl<'scope> KeyBindListener<'scope> {
    pub fn new<'env>(
        s: &'scope Scope<'scope, 'env>, e: Arc<RwLock<Engine<'env>>>,
        click_subscribers: Vec<Sender<(i16, i16, (Option<MouseButton>, Option<Key>))>>,
        keys_subscribers: Vec<Sender<Key>>,
        sidebar: Sender<SideBarMessage>
    ) -> Self {
        let arc = Arc::new(InterruptibleSleep::new());
        let sent = arc.clone();

        let save_dir = env::current_dir().unwrap().join("saves");

        let _ = fs::create_dir(&save_dir);

        let t = s.spawn(move || {
            let clicks = click_subscribers;
            let keys = keys_subscribers;
            let cop = e;
            let stdin = stdin();
            let stop_var = sent;

            for c in stdin.events() {
                if c.is_err() {
                    trace!("event error: {:?}", c.unwrap_err());
                    continue;
                }

                let event = c.unwrap();

                match &event {
                    Event::Key(Key::Left) => Self::offset_viewport(&cop, Key::Left),
                    Event::Key(Key::Right) => Self::offset_viewport(&cop, Key::Right),
                    Event::Key(Key::Up) => Self::offset_viewport(&cop, Key::Up),
                    Event::Key(Key::Down) => Self::offset_viewport(&cop, Key::Down),
                    Event::Key(Key::Char('q')) => break,
                    Event::Key(Key::Char('\n')) => {
                        for sender in &clicks {
                            let _ = sender.send((0,0, (None, Some(Key::Char('\n')))));
                        }
                    },
                    Event::Key(Key::Esc) => {
                        for sender in &clicks {
                            let _ = sender.send((0,0, (None, Some(Key::Esc))));
                        }
                    },
                    Event::Key(Key::Ctrl('s')) => {
                        match cop.read() {
                            Ok(ref engine) => {
                                let time = chrono::offset::Local::now();
                                
                                let name = save_dir.join(time.format("layout-%Y-%m-%d-%H-%M-%S.json").to_string());
                                
                                match serde_json::to_string(&engine.layout) {
                                    Ok(c) => {
                                        let _ = fs::write(name, c);
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    },
                    Event::Mouse(mouse_event) => match mouse_event {
                        MouseEvent::Press(click_type, x, y) => {
                            debug!("Mouse click at x: {} y: {} | {:?}", x, y, click_type);
                            if !(*click_type == MouseButton::WheelDown) {
                                match cop.write() {
                                    Ok(ref mut engine) => {
                                        let (virtual_x, virtual_y) =
                                            engine.viewport.get_virtual_coordinates(*x, *y);
                                        debug!("x: {virtual_x}, y: {virtual_y}");
                                        let d =
                                            engine.get_drawable_for_coordinates(virtual_x, virtual_y).map(|x| x.infos(engine.deref()));

                                        for click_sender in &clicks {
                                            let _ = click_sender.send((virtual_x, virtual_y, (Some(*click_type), None)));
                                        }

                                        if let Some(Some(x)) = d {
                                            send_to_side_bar_write(engine, SideBarMessage::CustomInfos(Box::new("Building infos"), x.iter().map(|x| Box::new(x.clone()) as Box<SyncDisplay>).collect::<Vec<Box<SyncDisplay>>>()));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    },
                    _ => {}
                };

                print!("{}", cursor::Goto(1, 1));
            }

            stop_var.stop();
            RUNNING.store(false, Ordering::SeqCst)
        });

        KeyBindListener {
            thread: t,
            stop_var: arc
        }
    }

    fn offset_viewport(e: &Arc<RwLock<Engine>>, key: Key) {
        match e.write() {
            Ok(mut guard) => {
                let e = &mut *guard;

                match key {
                    Key::Left => e.viewport.move_x(-4),
                    Key::Right => e.viewport.move_x(4),
                    Key::Up => e.viewport.move_y(-4),
                    Key::Down => e.viewport.move_y(4),
                    _ => {}
                }

                e.refresh()
            }
            Err(_) => {}
        }
    }
}
