use std::fmt::Display;
use std::thread::{sleep, Scope, ScopedJoinHandle};
use std::time::Duration;
use rand::prelude::SliceRandom;
use rand::rng;
use crate::engine::core::{Engine, LockableEngine};
use crate::POPULATION;
use crate::simulation::update_population;
use crate::ui::sidebar::{LogColor, LogType, SyncDisplay};
use crate::utils::unwrap_sidebar;

pub fn demo_scope<'scope, 'env>(s: &'scope Scope<'scope, 'env>, engine: LockableEngine) -> ScopedJoinHandle<'scope, ()> {
    s.spawn(move || {
        let engine = engine;
        let mut witness_dead = false;
        let mut rng = rng();

        for _ in 0..3 {
            unwrap_sidebar(&engine, |s| {
                s.push_log_and_display(Box::new("..."),
                                       LogType::Debug,
                                       LogColor::Normal)
            });

            sleep(Duration::from_millis(400));
        }

        unwrap_sidebar(&engine, |s| {
            s.push_log_and_display(Box::new("Begin..."),
                                   LogType::Debug,
                                   LogColor::Unusual)
        });

        sleep(Duration::from_secs(1));

        unwrap_sidebar(&engine, |s| {
            s.push_log_and_display(Box::new("Generating starting population..."),
                                   LogType::Debug,
                                   LogColor::Normal)
        });

        sleep(Duration::from_secs(1));

        unwrap_sidebar(&engine, |s| {
            s.push_log_and_display(Box::new("Adding 100 people into city..."),
                                   LogType::Debug,
                                   LogColor::Normal)
        });

        for i in 0..100 {

            unwrap_sidebar(&engine, |s| {
                s.push_log_and_display(Box::new(format!("_____YEAR {i}_____")),
                                       LogType::Debug,
                                       LogColor::Normal)
            });

            update_population(&engine, &mut POPULATION.lock().unwrap(), true);

            // sleep(Duration::from_millis(3600));

            let peoples = POPULATION.lock().unwrap().get_core_district().num_people;
            let deads = POPULATION
                .lock()
                .unwrap()
                .get_core_district()
                .get_population_number_by(crate::population::people::PeopleLegalState::Dead);

            unwrap_sidebar(&engine, |s| {
                s.push_log_and_display(Box::new(format!("New population : {}", peoples - deads)),
                                       LogType::Debug,
                                       LogColor::Unusual)
            });

            sleep(Duration::from_millis(500));

            if i % 10 == 0 {
                unwrap_sidebar(&engine, |s| {
                    s.push_log_and_display(Box::new("Displaying a random population member:"),
                                           LogType::Debug,
                                           LogColor::Unusual)
                });

                if witness_dead {
                    witness_dead = false;
                    POPULATION
                        .lock()
                        .unwrap()
                        .get_core_district_mut()
                        .peoples
                        .shuffle(&mut rng);
                }

                let people = POPULATION.lock().unwrap().get_core_district().peoples[0].clone();

                if people.as_alive() == None {
                    witness_dead = true;
                }

                let debug: String = format!("{:#?}", people);
                let lines = debug
                    .lines()
                    .map(|s| Box::new(s.to_string()) as Box<SyncDisplay>)
                    .collect();

                unwrap_sidebar(&engine, |s| {
                    s.push_multiline_log_and_display(lines,
                                                     LogType::None,
                                                     LogColor::Normal)
                });

                sleep(Duration::from_secs(2));
            }
        }
    })
}