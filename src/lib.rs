use std::sync::Mutex;
use lazy_static::lazy_static;
use log::trace;
use crate::engine::core::Engine;
use crate::engine::viewport::Viewport;
use crate::ui::sidebar::SideBar;

pub mod terminal;
pub mod logging;
pub mod ui;
pub mod engine;
pub mod population;
pub mod simulation;
pub mod city_layout;

lazy_static! {
    pub static ref SIDE_BAR: Mutex<SideBar> = Mutex::new(SideBar::new());
}



lazy_static! {
    pub static ref ENGINE: Mutex<Engine> = Mutex::new(Engine::new(Viewport::from_ratio(0.75, 1), STDOUT.deref()));
}