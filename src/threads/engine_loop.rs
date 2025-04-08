use std::any::Any;
use std::fmt::Display;
use std::ops::Deref;
use std::process::exit;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread::{sleep, Scope, ScopedJoinHandle};
use std::time::Duration;
use log::info;
use rand::prelude::SliceRandom;
use rand::rng;
use crate::engine::core::{Engine, LockableEngine};
use crate::{return_on_cancel, send_to_side_bar_auto, POPULATION};
use crate::engine::layout::{BuildingType, Layout};
use crate::simulation::update_population;
use crate::ui::sidebar::{LogColor, LogType, SyncDisplay};
use crate::utils::interruptible_sleep::InterruptibleSleep;
use crate::utils::send_to_side_bar;

pub fn engine_loop<'scope, 'env>(
    s: &'scope Scope<'scope, 'env>,
    engine: LockableEngine,
    stop_var: Arc<InterruptibleSleep>,
    receiver: Receiver<(i16, i16)>
) -> ScopedJoinHandle<'scope, ()> {
    s.spawn(move || {
        fn remove_building_from_coords(x: i16, y: i16, engine: &LockableEngine, filter: BuildingType) -> bool{
            let mut engine_write = engine.write().unwrap();
            let to_delete = {
                let drwbl = engine_write.layout.get_building_for_coordinates(x, y);
                if let Some(drwbl) = drwbl {
                    Option::from(drwbl.id)
                }
                else {
                    Option::None
                }
            };
            if let Some(to_del) = to_delete {
                engine_write.layout.replace_empty_building(to_del);
                engine_write.refresh();
                drop(engine_write);
                true
            }
            else {
                drop(engine_write);
                false
            }
        }

        for (x,y) in receiver {
            remove_building_from_coords(x,y, &engine, BuildingType::EmptySpace);
        }
        //let mut engine_write = engine.write().unwrap();


        return_on_cancel!(stop_var, Duration::from_secs(2));
    })
}