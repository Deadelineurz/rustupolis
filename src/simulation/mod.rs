use births::{number_of_children_to_make, spawn_childs};
use deaths::check_death;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::engine::core::{Engine, LockableEngine};
use crate::{
    engine::layout::Building,
    population::{
        district::PopulationDistrict,
        people::{BasePeopleInfo, People, PeopleLegalState},
        Population,
    },
    send_to_side_bar_auto,
    ui::sidebar::{LogColor, LogType},
};
pub mod births;
pub mod deaths;
pub mod dna_transmission;

/// Will forward one months \
/// Kinda expensive, will do a DFS on the districts then shuffle the population to make babies.
pub fn update_time_population(
    engine: &LockableEngine,
    population: &mut Population,
    birth_month: bool,
    rng: &mut ThreadRng,
    debug: bool,
) {
    let order = population.num_districts;
    let mut marks: Vec<bool> = vec![false; order];

    update_district_peoples(engine, 0, population, birth_month, &mut marks, rng, debug);
}

fn update_district_peoples(
    engine: &Arc<RwLock<Engine>>,
    district_id: usize,
    population: &mut Population,
    birth_month: bool,
    marked: &mut Vec<bool>,
    rng: &mut ThreadRng,
    debug: bool,
) {
    if !marked[district_id] {
        marked[district_id] = true;

        let district = population.get_district_mut(district_id).unwrap();

        district
            .peoples
            .iter_mut()
            .filter_map(|p| p.as_alive_mut())
            .for_each(|alive| {
                alive.age_in_months += 1;
                if let Some(name) = alive.get_witness_name() {
                    if alive.age_in_months % 120 == 0 {
                        send_to_side_bar_auto!(
                            engine,
                            format!("{} celebrate his {}th year!", name, alive.get_age()),
                            LogType::City,
                            LogColor::Normal
                        );
                    }
                }
            });

        if debug {
            send_to_side_bar_auto!(
                engine,
                "One month has passed",
                LogType::City,
                LogColor::Normal
            );
        }

        if birth_month {
            update_births(engine, district, rng, debug);
        }
        update_deaths(engine, district, rng, debug);

        // district.recalcul_happiness();

        for district_id in district.neighbors.clone() {
            update_district_peoples(
                engine,
                district_id,
                population,
                birth_month,
                marked,
                rng,
                debug,
            );
        }
    }
}

/// Will shuffle the district's population because of the parents
fn update_births(
    engine: &Arc<RwLock<Engine>>,
    district: &mut PopulationDistrict,
    rng: &mut ThreadRng,
    debug: bool,
) {
    let mut childs: Vec<People> = make_pairs(
        district
            .peoples
            .iter()
            .filter(|p| p.get_legal_state() != PeopleLegalState::Child && p.as_alive().is_some())
            .collect(),
        rng,
    )
    .iter()
    .map(|(parent1, parent2)| {
        let kids = spawn_childs(
            number_of_children_to_make(parent1.as_alive().unwrap(), district),
            parent1.as_alive().unwrap(),
            parent2.as_alive().unwrap(),
        );
        if kids.len() > 0 {
            if parent1.is_witness() {
                send_to_side_bar_auto!(
                    engine,
                    format!(
                        "{} had {} child",
                        parent1.get_witness_name().unwrap(),
                        kids.len()
                    ),
                    LogType::City,
                    LogColor::Normal
                );
            } else if parent2.is_witness() {
                send_to_side_bar_auto!(
                    engine,
                    format!(
                        "{} had {} child",
                        parent2.get_witness_name().unwrap(),
                        kids.len()
                    ),
                    LogType::City,
                    LogColor::Normal
                );
            }
        }

        kids
    })
    .collect::<Vec<_>>()
    .concat();

    if debug {
        send_to_side_bar_auto!(
            engine,
            format!("Births: {}", childs.len()),
            LogType::City,
            LogColor::Normal
        );
    }

    district.add_peoples(&mut childs);
}

fn update_deaths(
    engine: &LockableEngine,
    district: &mut PopulationDistrict,
    rng: &mut ThreadRng,
    debug: bool,
) {
    let zone = district.zone_type.clone();
    let happiness: f64 = district.get_happiness_percentage().into();

    let bef = district.get_population_number_by(PeopleLegalState::Dead);

    district.peoples.retain(|people| people.as_alive() != None); // clear corpse
    district.peoples.iter_mut().for_each(|people| {
        if let Some((cause, _)) = check_death(people.as_alive().unwrap(), zone, happiness, rng) {
            if people.is_witness() {
                send_to_side_bar_auto!(
                    engine,
                    format!(
                        "{} died {} at {}y.",
                        people.get_witness_name().unwrap(),
                        match cause {
                            crate::population::people::CauseOfDeath::OldAge => "of old age",
                            crate::population::people::CauseOfDeath::Murder => "killed by someone",
                            crate::population::people::CauseOfDeath::Sickness => "of a disease",
                            crate::population::people::CauseOfDeath::Radiations => "of radiations",
                            crate::population::people::CauseOfDeath::WorkAccident =>
                                "of a work accident",
                            crate::population::people::CauseOfDeath::EatenByMonster =>
                                "eaten by a monster",
                            crate::population::people::CauseOfDeath::Poverty => "because of hunger",
                        },
                        people.get_age()
                    ),
                    LogType::City,
                    LogColor::Important
                );
            }
            people.make_dead(cause);
        }
    });

    if debug {
        send_to_side_bar_auto!(
            engine,
            format!(
                "Deaths {}",
                district.get_population_number_by(PeopleLegalState::Dead) - bef
            ),
            LogType::City,
            LogColor::Normal
        );
    }
}

/// Ensure that the peoples are grouped by building uuid and only get selected once
fn make_pairs(people: Vec<&People>, rng: &mut ThreadRng) -> Vec<(People, People)> {
    let mut building_groups: HashMap<String, Vec<&People>> = HashMap::new();

    for person in people {
        building_groups
            .entry(
                person
                    .get_building_uuid()
                    .unwrap_or(&"homeless".to_string())
                    .clone(),
            )
            .or_default()
            .push(person);
    }

    let mut pairs = Vec::new();

    for (_building_uuid, guys_in_the_building) in building_groups.iter_mut() {
        guys_in_the_building.shuffle(rng);

        while guys_in_the_building.len() >= 2 {
            let p1 = guys_in_the_building.pop().unwrap();
            let p2 = guys_in_the_building.pop().unwrap();
            pairs.push((p1.clone(), p2.clone()));
        }
    }

    pairs
}

fn _update_people_in_building(
    _peoples: Vec<&People>,
    _rng: &mut ThreadRng,
    _buildings: &mut Building,
) {
    todo!();
}
