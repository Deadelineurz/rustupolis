use std::io::{stdin, stdout, Stdout, Write};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use log::trace;
use termion::cursor;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use crate::engine::core::Engine;
use crate::engine::drawable::{Drawable, DynDrawable};
use crate::engine::test::TestDrawable;
use crate::ui::sidebar::SideBar;

type Tty = MouseTerminal<RawTerminal<Stdout>>;

pub struct KeyBindListener<'a> {
    engine: Arc<Mutex<Engine<'a>>>,
    pub thread: JoinHandle<()>
}

impl KeyBindListener<'static> {
    pub fn new(e: Arc<Mutex<Engine<'static>>>, stdout: &'static Tty) -> Self {
        let cop = e.clone();

        let t = thread::spawn(move || {
            let stdin = stdin();
            let std = stdout;
            let mut side_bar = SideBar::new();
            side_bar.draw(std).expect("TODO: panic message");

            for c in stdin.events() {
                if c.is_err() {
                    trace!("event error: {:?}", c.unwrap_err());
                    continue
                }

                let event = c.unwrap();

                match &event {
                    Event::Key(Key::Left) => Self::offset_viewport(&cop, Key::Left),
                    Event::Key(Key::Right) => Self::offset_viewport(&cop, Key::Right),
                    Event::Key(Key::Up) => Self::offset_viewport(&cop, Key::Up),
                    Event::Key(Key::Down) => Self::offset_viewport(&cop, Key::Down),
                    Event::Key(Key::Char('q')) => break,
                    Event::Mouse(mouse_event) => {
                        match mouse_event {
                            MouseEvent::Press(_, x, y) => {
                                trace!("Mouse click at x: {} y: {}", x, y);
                                match &cop.lock() {
                                    Ok(guard) => {
                                        let (virt_x, virt_y) = guard.viewport.get_virtual_coordinates(*x, *y);
                                        let drawable: Option<&Box<DynDrawable>> = guard.get_drawable_for_coordinates(virt_x, virt_y);
                                        if drawable.is_none() {
                                            continue
                                        }
                                        let building = drawable.unwrap().downcast_ref::<TestDrawable>();
                                        if let Some(b) = building {
                                            let _ = side_bar.display_custom_infos(std,
                                                                                  &"Building infos:",
                                                                                  &[
                                                                                      &format!("PosX: {}", b.x()),
                                                                                      &format!("PosY: {}", b.y())
                                                                                  ]);
                                            trace!("{:?}", building);
                                        }
                                    }
                                    _ => ()
                                }
                            }
                            _ => ()
                        }
                    }
                    _ => {}
                };

                print!("{}", cursor::Goto(1, 1));
            }
        });

        KeyBindListener{
            engine: e,
            thread: t
        }
    }

   fn offset_viewport(e: &Arc<Mutex<Engine>>, key: Key) {
       match e.lock() {
           Ok(mut guard) => {
               let e = &mut *guard;

               match key {
                   Key::Left => e.viewport.move_x(-1),
                   Key::Right => e.viewport.move_x(1),
                   Key::Up => e.viewport.move_y(-1),
                   Key::Down => e.viewport.move_y(1),
                   _ => {}
               }

               e.refresh()
           }
           Err(_) => {}
       }
   }
}