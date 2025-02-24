use crate::logging::RemoteLoggerClient;
use lazy_static::lazy_static;
use log::LevelFilter;
use rand::rng;
use rand::seq::SliceRandom;
use rustupolis::engine::core::Engine;
use rustupolis::engine::keybinds::KeyBindListener;
use rustupolis::engine::layout::Layout;
use rustupolis::engine::viewport::Viewport;
use rustupolis::simulation::update_population;
use rustupolis::terminal::screen::CleanScreen;
use rustupolis::ui::sidebar::{LogColor, LogType};
use rustupolis::{POPULATION, SIDE_BAR, STDOUT};
use std::fmt::Display;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use termion::terminal_size;

mod logging;

lazy_static! {
    pub static ref LOGGER: RemoteLoggerClient = RemoteLoggerClient::new();

}

fn main() {
    log::set_logger(LOGGER.deref())
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();

    let _clear = CleanScreen::new();

    let layout = Layout::load_default_layout();

    let bdrawables = layout.get_buildings();
    let rdrawables = layout.get_roads();

    let mut vp = Viewport::default();

    vp.width = (terminal_size().unwrap().0 as f32 * 0.75) as u16;

    let mut engine = Engine::new(vp, STDOUT.deref());

    for d in bdrawables {
        engine.register_drawable(Box::new(d));
    }

    for d in rdrawables {
        engine.register_drawable(Box::new(d));
    }

    engine.refresh();

    let e = Arc::new(Mutex::new(engine));

    let kb_engine_ref = e.clone();

    let kb = KeyBindListener::new(kb_engine_ref, STDOUT.deref());

    SIDE_BAR.lock().unwrap().draw(STDOUT.deref()).unwrap();

    // bien bien dégeu mais au moins on a une démo sympa

    let mut witness_dead = false;
    let mut rng = rng();

    for _ in 0..3 {
        SIDE_BAR
            .lock()
            .unwrap()
            .push_log_and_display(
                STDOUT.deref(),
                Box::new("..."),
                LogType::Debug,
                LogColor::Normal,
            )
            .unwrap();

        sleep(Duration::from_millis(400));
    }

    SIDE_BAR
        .lock()
        .unwrap()
        .push_log_and_display(
            STDOUT.deref(),
            Box::new("Begin..."),
            LogType::Debug,
            LogColor::Unusual,
        )
        .unwrap();

    sleep(Duration::from_secs(1));

    SIDE_BAR
        .lock()
        .unwrap()
        .push_log_and_display(
            STDOUT.deref(),
            Box::new("Generating starting population..."),
            LogType::Debug,
            LogColor::Normal,
        )
        .unwrap();

    sleep(Duration::from_secs(1));

    SIDE_BAR
        .lock()
        .unwrap()
        .push_log_and_display(
            STDOUT.deref(),
            Box::new("Adding 80 people into city..."),
            LogType::Debug,
            LogColor::Normal,
        )
        .unwrap();

    POPULATION.lock().unwrap().add_peoples(80, None);

    for i in 0..100 {
        SIDE_BAR
            .lock()
            .unwrap()
            .push_log_and_display(
                STDOUT.deref(),
                Box::new(format!("_____YEAR {i}_____")),
                LogType::Debug,
                LogColor::Normal,
            )
            .unwrap();

        update_population(
            &mut POPULATION.lock().unwrap(), true
        );

        sleep(Duration::from_millis(3600));

        let peoples = POPULATION.lock().unwrap().get_core_district().num_people;
        let deads = POPULATION
            .lock()
            .unwrap()
            .get_core_district()
            .get_population_number_by(rustupolis::population::people::PeopleLegalState::Dead);

        SIDE_BAR
            .lock()
            .unwrap()
            .push_log_and_display(
                STDOUT.deref(),
                Box::new(format!("New population : {}", peoples - deads)),
                LogType::Debug,
                LogColor::Unusual,
            )
            .unwrap();

        sleep(Duration::from_millis(500));

        if i % 10 == 0 {
            SIDE_BAR
                .lock()
                .unwrap()
                .push_log_and_display(
                    STDOUT.deref(),
                    Box::new("Displaying a random population member:"),
                    LogType::Debug,
                    LogColor::Unusual,
                )
                .unwrap();

            if witness_dead {
                witness_dead = false;
                POPULATION.lock().unwrap().get_core_district_mut().peoples.shuffle(&mut rng);
            }

            let people = POPULATION.lock().unwrap().get_core_district().peoples[0].clone();

            if people.as_alive() == None {
                witness_dead = true;
            }

            let debug: String = format!("{:#?}", people);
            let lines = debug
                .lines()
                .map(|s| Box::new(s.to_string()) as Box<dyn Display + Send + 'static>)
                .collect();

            SIDE_BAR
                .lock()
                .unwrap()
                .push_multiline_log_and_display(
                    STDOUT.deref(),
                    lines,
                    LogType::None,
                    LogColor::Normal,
                )
                .unwrap();

                sleep(Duration::from_secs(2));
        }
    }

    let _ = kb.thread.join();

    drop(_clear);
}
