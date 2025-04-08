use crate::engine::core::LockableEngine;
use crate::engine::layout::BuildingType;
use crate::utils::interruptible_sleep::InterruptibleSleep;
use crate::return_on_cancel;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::{Scope, ScopedJoinHandle};
use std::time::Duration;

pub fn engine_loop<'scope, 'env>(
    s: &'scope Scope<'scope, 'env>,
    engine: LockableEngine,
    stop_var: Arc<InterruptibleSleep>,
    receiver: Receiver<(i16, i16)>
) -> ScopedJoinHandle<'scope, ()> {
    s.spawn(move || {
        fn remove_building_from_coords(x: i16, y: i16, engine: &LockableEngine, _filter: BuildingType) -> bool{
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