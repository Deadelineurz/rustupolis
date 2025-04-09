use std::ops::Deref;
use crate::engine::keybinds::{Tty, RUNNING};
use crate::ui::sidebar::{LogColor, LogType, SideBar, SyncDisplay};
use std::sync::atomic::Ordering;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub enum SideBarMessage {
    Single(Box<SyncDisplay>, LogType, LogColor),
    Multiple(Vec<Box<SyncDisplay>>, LogType, LogColor),
    CustomInfos(Box<SyncDisplay>, Vec<Box<SyncDisplay>>),
    Quit
}


pub fn sidebar(t: Arc<Tty>) -> (Sender<SideBarMessage>, JoinHandle<()>) {
    let (tx, rx) = channel::<SideBarMessage>();

    let x = thread::spawn(move || {
        let mut sidebar = SideBar::new(t);
        let _ = sidebar.draw();
        while let Ok(m) = rx.recv() {
            let _ = match m {
                SideBarMessage::Single(msg, t, col) => {
                    sidebar.push_log_and_display(msg, t, col)
                }
                SideBarMessage::Multiple(msg, t, col) => {
                    sidebar.push_multiline_log_and_display(msg, t, col)
                }
                SideBarMessage::CustomInfos(header, infos) => {
                    sidebar.display_custom_infos(header.deref(), &infos.iter().map(|x| x.deref()).collect::<Vec<&SyncDisplay>>())
                }
                SideBarMessage::Quit => {
                    return
                }
            };

            if !RUNNING.load(Ordering::SeqCst) {
                break
            }
        }
    });

    (tx, x)
}

