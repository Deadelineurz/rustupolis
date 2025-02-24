use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use population::Population;
use crate::ui::sidebar::SideBar;

pub mod terminal;
pub mod logging;
pub mod ui;
pub mod engine;
pub mod population;
pub mod simulation;

lazy_static! {
    pub static ref SIDE_BAR: Arc<Mutex<SideBar>> = Arc::new(Mutex::new(SideBar::new()));
    pub static ref POPULATION: Arc<Mutex<Population>> = Arc::new(Mutex::new(Population::new()));
}