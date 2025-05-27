use crate::engine::core::LockableEngine;
use crate::simulation::update_time_population;
use crate::ui::sidebar::{LogColor, LogType};
use crate::ui::topbar::TopBar;
use crate::utils::interruptible_sleep::InterruptibleSleep;
use crate::utils::{send_to_side_bar_read};
use crate::{lock_read, lock_unlock, lock_write, return_on_cancel, send_to_side_bar_auto};
use rand::prelude::SliceRandom;
use rand::rng;
use std::sync::Arc;
use std::thread::{Scope, ScopedJoinHandle};
use std::time::Duration;
use termion::terminal_size;
use crate::threads::sidebar::SideBarMessage;

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

        send_to_side_bar_auto!(e, engine,
            "Begin...",
            LogType::Debug,
            LogColor::Unusual);

        return_on_cancel!(stop_var, Duration::from_millis(500));

        send_to_side_bar_auto!(e, engine,
            "Generating starting population...",
            LogType::Debug,
            LogColor::Normal);

        let mut refresh = 0;
	let mut witnesses_to_birth: u8 = 0;
        for i in 0..12000 {
            let _ = topbar.update_displayed_year(i / 12);
            update_time_population(
                &engine,
                i % 12 == 0,
                &mut witnesses_to_birth,
                &mut rng,
                false,
            );
            lock_read!(engine |> pop);

            let core_district = pop.population.get_core_district();

            let peoples = core_district.num_people;
            let workers = core_district.working_poulation;

            let _ = topbar.update_displayed_population(peoples);

            let _ = topbar.update_displayed_happiness(core_district.get_happiness_percentage());

            let _ = topbar.update_displayed_workers(workers, peoples);

            lock_unlock!(pop);

            if i % 36 == 0 {
                lock_write!(engine |> generator);

                generate_next_step(&mut generator.layout, &mut rng);

                lock_unlock!(generator);
            }

            if refresh == 20 {
                lock_write!(engine |> e);
                refresh = 0;
                e.refresh();
                lock_unlock!(e);
            }

            return_on_cancel!(stop_var, Duration::from_millis(50));
            refresh += 1;

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
