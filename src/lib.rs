use std::{io::stdout, sync::{Arc, Mutex}};
use engine::{keybinds::Tty, layout::Layout};
use lazy_static::lazy_static;
use population::Population;
use termion::{input::MouseTerminal, raw::IntoRawMode};
use crate::ui::sidebar::SideBar;

pub mod terminal;
pub mod logging;
pub mod ui;
pub mod engine;
pub mod population;
pub mod simulation;
pub mod threads;
pub mod utils;

lazy_static! {
    pub static ref POPULATION: Arc<Mutex<Population>> = Arc::new(Mutex::new(Population::new()));
    pub static ref LAYOUT: Arc<Mutex<Layout>> = Arc::new(Mutex::new(Layout::load_default_layout()));
}