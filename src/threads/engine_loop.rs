use crate::engine::core::{Engine, LockableEngine};
use crate::engine::layout::{Building, BuildingType};
use crate::utils::interruptible_sleep::InterruptibleSleep;
use crate::return_on_cancel;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::{Scope, ScopedJoinHandle};
use std::time::Duration;
use log::{debug, trace};
use termion::event::Key::Left;
use termion::event::MouseButton;

pub fn engine_loop<'scope, 'env>(
    s: &'scope Scope<'scope, 'env>,
    engine: LockableEngine,
    stop_var: Arc<InterruptibleSleep>,
    receiver: Receiver<(i16, i16, MouseButton)>
) -> ScopedJoinHandle<'scope, ()> {
    s.spawn(move || {
        let mut inputs = vec![];
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

        pub fn add_building_from_coords(x: i16, y: i16, width: u8, height: u8, engine: &LockableEngine) {
            let mut engine = engine.write().unwrap();
            engine.layout.add_building_from_coords(x, y, width, height);
            engine.refresh();
        }

        for (x,y,click_type) in receiver {
            inputs.push((x,y, click_type));
            debug!("{:?}", inputs);
            //
            if click_type == MouseButton::Right {
                add_building_from_coords(x,y,10,10, &engine);
            }
            else {
                remove_building_from_coords(x,y, &engine, BuildingType::Uniform);
            }
        }
        //let mut engine_write = engine.write().unwrap();


    })
}