use std::fmt::Display;
use std::ops::Deref;
use std::process::exit;
use std::sync::Arc;
use std::thread::{sleep, Scope, ScopedJoinHandle};
use std::time::Duration;
use log::info;
use rand::prelude::SliceRandom;
use rand::rng;
use crate::engine::core::{Engine, LockableEngine};
use crate::{return_on_cancel, send_to_side_bar_auto, POPULATION};
use crate::engine::layout::Layout;
use crate::simulation::update_population;
use crate::ui::sidebar::{LogColor, LogType, SyncDisplay};
use crate::utils::interruptible_sleep::InterruptibleSleep;
use crate::utils::send_to_side_bar;

pub fn engine_loop<'scope, 'env>(s: &'scope Scope<'scope, 'env>, engine: LockableEngine, stop_var: Arc<InterruptibleSleep>) -> ScopedJoinHandle<'scope, ()> {
    s.spawn(move || {
        let mut engine_write = engine.write().unwrap();
        let layout2 = Layout::load_default_layout2();
        let bdrawables2 = layout2.get_buildings();


        for d in bdrawables2 {
            engine_write.replace_empty_drawable(Box::new(d));
        }

        return_on_cancel!(stop_var, Duration::from_secs(2));
    })
}