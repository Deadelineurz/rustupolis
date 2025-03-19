use crate::engine::core::Engine;
use crate::ui::sidebar::SyncDisplay;
use log::trace;
use std::fmt::Display;
use std::io::{stdin, stdout, Stdout};
use std::ops::Deref;
use std::process::exit;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::{JoinHandle, Scope, ScopedJoinHandle};
use termion::cursor;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::RawTerminal;

pub type Tty = MouseTerminal<RawTerminal<Stdout>>;

pub trait Clickable {
    fn infos(&self) -> Option<Vec<String>> {
        None
    }
}

pub struct KeyBindListener<'scope> {
    pub thread: ScopedJoinHandle<'scope, ()>,
}

pub static RUNNING: AtomicBool = AtomicBool::new(true);

impl<'scope> KeyBindListener<'scope> {
    pub fn new<'env>(s: &'scope Scope<'scope, 'env>, e: Arc<RwLock<Engine>> ) -> Self {
        let t = s.spawn(move || {
            let cop = e;
            let stdin = stdin();

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
                    Event::Mouse(mouse_event) => match mouse_event {
                        MouseEvent::Press(_, x, y) => {
                            trace!("Mouse click at x: {} y: {}", x, y);
                            match cop.write() {
                                Ok(ref mut engine) => {
                                    let (virtual_x, virtual_y) =
                                        engine.viewport.get_virtual_coordinates(*x, *y);
                                    let d =
                                        engine.get_drawable_for_coordinates(virtual_x, virtual_y);
                                    if d.is_none() {
                                        continue;
                                    }

                                    let infos = d.unwrap().infos();

                                    if infos.is_none() {
                                        continue;
                                    }

                                    match engine.sidebar {
                                        Some(ref mut sb) => {
                                            let s = infos.unwrap();
                                            let _ = sb.display_custom_infos(
                                                &"Building infos",
                                                s.iter()
                                                    .map(|s| s as &SyncDisplay)
                                                    .collect::<Vec<&SyncDisplay>>()
                                                    .as_slice(),
                                            );
                                        }
                                        _ => (),
                                    }
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    },
                    _ => {}
                };

                print!("{}", cursor::Goto(1, 1));
            }

            RUNNING.store(true, Ordering::SeqCst)
        });

        KeyBindListener {
            thread: t,
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
