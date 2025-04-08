use crate::engine::core::LockableEngine;
use crate::simulation::update_population;
use crate::ui::sidebar::{LogColor, LogType, SyncDisplay};
use crate::utils::interruptible_sleep::InterruptibleSleep;
use crate::utils::{send_to_side_bar_read};
use crate::{lock_read, lock_unlock, lock_write, return_on_cancel, send_to_side_bar_auto};
use rand::prelude::SliceRandom;
use rand::rng;
use std::sync::Arc;
use std::thread::{Scope, ScopedJoinHandle};
use std::time::Duration;

pub fn demo_scope<'scope, 'env>(s: &'scope Scope<'scope, 'env>, engine: LockableEngine, stop_var: Arc<InterruptibleSleep>) -> ScopedJoinHandle<'scope, ()> {
    s.spawn(move || {
        let engine = engine;
        let mut witness_dead = false;
        let mut rng = rng();
        /*for _ in 0..3 {
            send_to_side_bar(&engine, (Box::new("..."),
                                      LogType::Debug,
                                      LogColor::Normal));

            sleep(Duration::from_millis(400));
        }*/

        send_to_side_bar_auto!(e, engine,
            "Begin...",
            LogType::Debug,
            LogColor::Unusual);

        if !stop_var.wait_for(Duration::from_secs(1)) {
            return
        }

        send_to_side_bar_auto!(e, engine,
            "Generating starting population..." ; "Adding 100 people into city...",
            LogType::Debug,
            LogColor::Normal);

        for i in 0..100 {

            send_to_side_bar_auto!(e, engine,
                format!("_____YEAR {i}_____"),
                LogType::Debug,
                LogColor::Normal);

            update_population(&engine, true);

            // sleep(Duration::from_millis(3600));

            lock_read!(engine |> e);

            let peoples = e.population.get_core_district().num_people;
            let deads = e
                .population
                .get_core_district()
                .get_population_number_by(crate::population::people::PeopleLegalState::Dead);

            send_to_side_bar_auto!(r, &e, format!("New population : {}", peoples - deads), LogType::Debug, LogColor::Unusual);
            lock_unlock!(e);

            return_on_cancel!(stop_var, Duration::from_millis(500));

            if i % 10 == 0 {
                send_to_side_bar_auto!(e, engine, "Displaying a random population member:", LogType::Debug, LogColor::Unusual);

                if witness_dead {
                    witness_dead = false;
                    lock_write!(engine |> w_engine);
                    w_engine
                        .population
                        .get_core_district_mut()
                        .peoples
                        .shuffle(&mut rng);
                }

                let people = engine.read().unwrap().population.get_core_district().peoples[0].clone();

                if people.as_alive() == None {
                    witness_dead = true;
                }

                let debug: String = format!("{:#?}", people);
                let lines = debug
                    .lines()
                    .map(|s| Box::new(s.to_string()) as Box<SyncDisplay>)
                    .collect();

                lock_read!(engine |> x);

                send_to_side_bar_read(&x, (lines, LogType::None, LogColor::Normal));

                lock_unlock!(x);
                return_on_cancel!(stop_var, Duration::from_secs(2));
            }
        }
    })
}