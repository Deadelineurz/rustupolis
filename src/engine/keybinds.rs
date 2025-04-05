use crate::engine::core::Engine;
use crate::utils::interruptible_sleep::InterruptibleSleep;
use log::trace;
use std::io::{stdin, Stdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::{Scope, ScopedJoinHandle};
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
    pub stop_var: Arc<InterruptibleSleep>
}

pub static RUNNING: AtomicBool = AtomicBool::new(true);

impl<'scope> KeyBindListener<'scope> {
    pub fn new<'env>(s: &'scope Scope<'scope, 'env>, e: Arc<RwLock<Engine>> ) -> Self {
        let arc = Arc::new(InterruptibleSleep::new());
        let sent = arc.clone();

        let t = s.spawn(move || {
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

                                    let _s = infos.unwrap();
                                    /*let _ = engine.sidebar.display_custom_infos(
                                        &"Building infos",
                                        s.iter()
                                            .map(|s| s as &SyncDisplay)
                                            .collect::<Vec<&SyncDisplay>>()
                                            .as_slice(),
                                    );*/
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
