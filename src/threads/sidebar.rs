use crate::engine::keybinds::{Tty, RUNNING};
use crate::ui::sidebar::{LogColor, LogType, SideBar, SyncDisplay};
use std::sync::atomic::Ordering;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub type SideBarMessage = (Vec<Box<SyncDisplay>>, LogType, LogColor);

pub fn sidebar(t: Arc<Tty>) -> (Sender<SideBarMessage>, JoinHandle<()>) {
    let (tx, rx) = channel::<SideBarMessage>();

    let x = thread::spawn(move || {
        let mut sidebar = SideBar::new(t);
        let _ = sidebar.draw();
        while let Ok(m) = rx.recv() {
            let _ = sidebar.push_multiline_log_and_display(m.0, m.1, m.2);

            if !RUNNING.load(Ordering::SeqCst) {
                break
            }
        }
    });

    (tx, x)
}

