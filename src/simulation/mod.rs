use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use deaths::check_death;
use rand::{rng, rngs::ThreadRng, seq::SliceRandom};
use spawn_child::{number_of_children_to_make, spawn_childs};

use crate::{engine::layout::Building, population::{
    district::PopulationDistrict,
    people::{BasePeopleInfo, People, PeopleLegalState},
    Population,
}, send_to_side_bar_auto, ui::sidebar::{LogColor, LogType}};
use crate::engine::core::{Engine, LockableEngine};
use crate::utils::send_to_side_bar;

pub mod deaths;
pub mod dna_transmission;
pub mod spawn_child;

/// Kinda expensive, will do a DFS on the districts then shuffle the population to make babies.
pub fn update_population(engine: &LockableEngine, population: &mut Population, debug: bool) {
    let order = population.num_districts;
    let mut marks: Vec<bool> = vec![false; order];

    let mut rng = rng();

    update_district_peoples(engine, 0, population, &mut marks, &mut rng, debug);
}

fn update_district_peoples(
    engine: &Arc<RwLock<Engine>>,
    district_id: usize,
    population: &mut Population,
    marked: &mut Vec<bool>,
    rng: &mut ThreadRng,
    debug: bool
) {
    if !marked[district_id] {
        marked[district_id] = true;

        let district = population.get_district_mut(district_id).unwrap();

        district
            .peoples
            .iter_mut()
            .filter_map(|p| p.as_alive_mut())
            .for_each(|alive| alive.age += 1);

            if debug {
                send_to_side_bar_auto!(engine, "One year has passed", LogType::City, LogColor::Normal);
            }

        update_births(engine, district, rng, debug);
        update_deaths(engine, district, debug);

        for district_id in district.neighbors.clone() {
            update_district_peoples(engine, district_id, population, marked, rng, debug);
        }
    }
}

/// Will shuffle the district's population because of the parents
fn update_births(
    engine: &Arc<RwLock<Engine>>,
    district: &mut PopulationDistrict,
    rng: &mut ThreadRng,
    debug: bool
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
        spawn_childs(
            number_of_children_to_make(parent1.as_alive().unwrap(), district),
            parent1.as_alive().unwrap(),
            parent2.as_alive().unwrap(),
        )
    })
    .collect::<Vec<_>>()
    .concat();

    if debug {
        send_to_side_bar_auto!(engine, format!("Births: {}", childs.len()), LogType::City, LogColor::Normal);
    }

    district.add_peoples(&mut childs);
}



fn update_deaths(engine: &LockableEngine, district: &mut PopulationDistrict, debug: bool) {
    let zone = district.zone_type.clone();
    let happiness: f64 = district.get_happiness_percentage().into();

    let bef = district.get_population_number_by(PeopleLegalState::Dead);

    district
        .peoples
        .iter_mut()
        .filter(|people| people.as_alive() != None)
        .for_each(|people| {
            if let Some(cause) = check_death(people.as_alive().unwrap(), zone, happiness) {
                people.make_dead(cause);
            }
        });

    if debug {
        send_to_side_bar_auto!(engine,
            format!("Deaths {}", district.get_population_number_by(PeopleLegalState::Dead) - bef),
            LogType::City,
            LogColor::Normal);
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

fn _update_people_in_building(_peoples: Vec<&People>, _rng: &mut ThreadRng, _buildings: &mut Building) {
    todo!();
}
