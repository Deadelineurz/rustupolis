use crate::engine::core::LockableEngine;
use crate::simulation::update_time_population;
use crate::ui::sidebar::{LogColor, LogType};
use crate::ui::topbar::TopBar;
use crate::utils::interruptible_sleep::InterruptibleSleep;
use crate::{return_on_cancel, send_to_side_bar_auto, POPULATION};
use rand::rng;
use std::sync::Arc;
use std::thread::{Scope, ScopedJoinHandle};
use std::time::Duration;
use termion::terminal_size;

pub fn demo_scope<'scope, 'env>(
    s: &'scope Scope<'scope, 'env>,
    engine: LockableEngine,
    stop_var: Arc<InterruptibleSleep>,
) -> ScopedJoinHandle<'scope, ()> {
    s.spawn(move || {
        let engine = engine;
        let mut rng = rng();

        let (ter_x, _ter_y) = terminal_size().unwrap();
        let sidebar_width_offset = (ter_x as f32 * 0.75) as u16;

        let topbar = TopBar::new(
            engine.read().unwrap().stdout.clone(),
            ter_x - sidebar_width_offset - 1,
        );
        topbar.draw().unwrap();

        send_to_side_bar_auto!(&engine, "Begin...", LogType::Debug, LogColor::Unusual);

        if !stop_var.wait_for(Duration::from_secs(1)) {
            return;
        }

        send_to_side_bar_auto!(&engine,
            "Generating starting population..." ; "Adding 100 people into city...",
            LogType::Debug,
            LogColor::Normal);

        for i in 0..1200 {
            let _ = topbar.update_displayed_year(i / 12);

            update_time_population(
                &engine,
                &mut POPULATION.lock().unwrap(),
                i % 12 == 0,
                &mut rng,
                false,
            );

            let mutex = POPULATION.lock().unwrap();
            let core_district = mutex.get_core_district();

            let peoples = core_district.num_people;
            let workers = core_district.working_poulation;

            let _ = topbar.update_displayed_population(peoples);

            let _ = topbar.update_displayed_happiness(core_district.get_happiness_percentage());

            let _ = topbar.update_displayed_workers(workers, peoples);

            return_on_cancel!(stop_var, Duration::from_millis(50));

            // if i % 10 == 0 {
            //     send_to_side_bar_auto!(&engine, "Displaying a random population member:", LogType::Debug, LogColor::Unusual);

            //     if witness_dead {
            //         witness_dead = false;
            //         POPULATION
            //             .lock()
            //             .unwrap()
            //             .get_core_district_mut()
            //             .peoples
            //             .shuffle(&mut rng);
            //     }

            //     let people = POPULATION.lock().unwrap().get_core_district().peoples[0].clone();

            //     if people.as_alive() == None {
            //         witness_dead = true;
            //     }

            //     let debug: String = format!("{:#?}", people);
            //     let lines = debug
            //         .lines()
            //         .map(|s| Box::new(s.to_string()) as Box<SyncDisplay>)
            //         .collect();

            //     send_to_side_bar(&engine, (lines, LogType::None, LogColor::Normal));

            //     return_on_cancel!(stop_var, Duration::from_secs(2));
            // }
        }
    })
}
