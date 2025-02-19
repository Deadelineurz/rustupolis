use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use termion::event::Key;
use termion::input::TermRead;
use crate::engine::core::Engine;

pub struct KeyBindListener {
    engine: Arc<Mutex<Engine>>,
    pub thread: JoinHandle<()>
}

impl KeyBindListener {
    pub fn new(e: Arc<Mutex<Engine>>) -> Self {
        let cop = e.clone();

        let t = thread::spawn(move || {
            let stdin = stdin();

            for key in stdin.keys() {
                if key.is_err() {
                    continue
                }

                match &key.unwrap() {
                    Key::Left => Self::offset_viewport(&cop, Key::Left),
                    Key::Right => Self::offset_viewport(&cop, Key::Right),
                    Key::Up => Self::offset_viewport(&cop, Key::Up),
                    Key::Down => Self::offset_viewport(&cop, Key::Down),
                    Key::Char('q') => break,
                    _ => {}
                };
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