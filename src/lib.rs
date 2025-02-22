use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::ui::sidebar::SideBar;

pub mod terminal;
pub mod logging;
pub mod ui;
pub mod engine;
pub mod population;
pub mod simulation;

lazy_static! {
    pub static ref SIDE_BAR: Mutex<SideBar> = Mutex::new(SideBar::new());
}