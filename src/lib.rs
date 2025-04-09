use engine::layout::Layout;
use lazy_static::lazy_static;
use population::Population;
use std::sync::{Arc, Mutex};

pub mod terminal;
pub mod logging;
pub mod ui;
pub mod engine;
pub mod population;
pub mod simulation;
pub mod threads;
pub mod utils;
pub mod roads;

lazy_static! {
    pub static ref LAYOUT: Arc<Mutex<Layout>> = Arc::new(Mutex::new(Layout::load_default_layout()));
}