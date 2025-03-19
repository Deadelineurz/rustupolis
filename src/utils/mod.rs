use std::io::Error;
use crate::engine::core::LockableEngine;
use crate::ui::sidebar::SideBar;

pub fn unwrap_sidebar<F>(engine: &LockableEngine, action: F)
where F: FnOnce(&mut SideBar) -> Result<(), Error> {
    match engine.write() {
        Ok(ref mut e) => {
            let _ = action(&mut e.sidebar);
        }

        _ => {}
    }
}