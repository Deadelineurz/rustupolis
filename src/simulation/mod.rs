use crate::engine::core::{Engine, LockableEngine};
use crate::engine::layout::{Layout, LayoutId};
use crate::population::people::AlivePerson;
use crate::population::{self, Population};
use crate::threads::sidebar::SideBarMessage;
use crate::{
    engine::layout::Building,
    lock_read, lock_unlock, lock_write,
    population::{
        district::PopulationDistrict,
        people::{BasePeopleInfo, People, PeopleLegalState},
    },
    send_to_side_bar_auto,
    ui::sidebar::{LogColor, LogType},
};
use births::{number_of_children_to_make, spawn_childs};
use deaths::check_death;
use log::debug;
use rand::seq::IndexedRandom;
use rand::Rng;
use rand::{rng, rngs::ThreadRng, seq::SliceRandom};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};

pub mod births;
pub mod deaths;
pub mod dna_transmission;

/// Will forward one months \
/// Kinda expensive, will do a DFS on the districts then shuffle the population to make babies.
pub fn update_time_population(
    engine: &LockableEngine,
    birth_month: bool,
    witness_to_make: &mut u8,
    rng: &mut ThreadRng,
    debug: bool,
) {
    lock_read!(engine |> pop);
    let order = pop.population.num_districts;
    lock_unlock!(pop);
    let mut marks: Vec<bool> = vec![false; order];

    update_district_peoples(
        engine,
        0,
        birth_month,
        witness_to_make,
        &mut marks,
        rng,
        debug,
    )
}

fn update_district_peoples(
    engine: &Arc<RwLock<Engine>>,
    district_id: usize,
    birth_month: bool,
    witness_to_make: &mut u8,
    marked: &mut Vec<bool>,
    rng: &mut ThreadRng,
    debug: bool,
) {
    lock_write!(engine |> pop);
    if !marked[district_id] {
        marked[district_id] = true;
        let s = pop.side_bar_tx.clone();

        let district = pop.population.get_district_mut(district_id).unwrap();

        district
            .peoples
            .iter_mut()
            .filter_map(|p| p.as_alive_mut())
            .for_each(|alive| {
                alive.age_in_months += 1;
                if let Some(name) = alive.get_witness_name() {
                    if alive.age_in_months % 120 == 0 {
                        let _ = s.clone().send(SideBarMessage::Single(
                            Box::new(format!(
                                "{} celebrate his {}th year!",
                                name,
                                alive.get_age()
                            )),
                            LogType::City,
                            LogColor::Normal,
                        ));
                    }
                }
            });

        if birth_month {
            update_births(s.clone(), district, witness_to_make, rng, debug);
        }
        *witness_to_make += update_deaths(s.clone(), district, debug);

        let clones = district.neighbors.clone();
        // pop.refresh();
        lock_unlock!(pop);

        if debug {
            send_to_side_bar_auto!(
                e,
                engine,
                "One year has passed",
                LogType::City,
                LogColor::Normal
            );
        }

        for district_id in clones {
            update_district_peoples(
                engine,
                district_id,
                birth_month,
                witness_to_make,
                marked,
                rng,
                debug,
            );
        }
    }
}

/// Will shuffle the district's population because of the parents
fn update_births(
    pipe: Sender<SideBarMessage>,
    district: &mut PopulationDistrict,
    witness_to_make: &mut u8,
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
            if *witness_to_make > 0 {
                *witness_to_make -= 1;
                true
            } else {
                false
            },
            parent1.as_alive().unwrap(),
            parent2.as_alive().unwrap(),
        );
        if kids.len() > 0 {
            if parent1.is_witness() {
                let _ = pipe.send(SideBarMessage::Single(
                    Box::new(format!(
                        "{} had {} child",
                        parent1.get_witness_name().unwrap(),
                        kids.len()
                    )),
                    LogType::City,
                    LogColor::Normal,
                ));
            } else if parent2.is_witness() {
                let _ = pipe.send(SideBarMessage::Single(
                    Box::new(format!(
                        "{} had {} child",
                        parent2.get_witness_name().unwrap(),
                        kids.len()
                    )),
                    LogType::City,
                    LogColor::Normal,
                ));
            }
        }

        kids
    })
    .collect::<Vec<_>>()
    .concat();

    if debug {
        let _ = pipe.send(SideBarMessage::Single(
            Box::new(format!("Births: {}", childs.len())),
            LogType::City,
            LogColor::Normal,
        ));
    }

    district.add_peoples(&mut childs);
}

fn update_deaths(
    pipe: Sender<SideBarMessage>,
    district: &mut PopulationDistrict,
    debug: bool,
) -> u8 {
    let zone = district.zone_type.clone();
    let happiness: f64 = district.get_happiness_percentage().into();

    let bef = district.get_population_number_by(PeopleLegalState::Dead);

    let mut witness_to_make = 0;

    district.peoples.retain(|people| people.as_alive() != None); // clear corpse
    district.peoples.iter_mut().for_each(|people| {
        let mut rng = rng();
        if let Some((cause, _)) = check_death(people.as_alive().unwrap(), zone, happiness, &mut rng)
        {
            if people.is_witness() {
                let _ = pipe.send(SideBarMessage::Single(
                    Box::new(format!(
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
                    )),
                    LogType::City,
                    LogColor::Important,
                ));
                witness_to_make += 1;
            }
            people.make_dead(cause);
        }
    });

    if debug {
        let _ = pipe.send(SideBarMessage::Single(
            Box::new(format!(
                "Deaths {}",
                district.get_population_number_by(PeopleLegalState::Dead) - bef
            )),
            LogType::City,
            LogColor::Normal,
        ));
    }
    witness_to_make
}

/// Ensure that the peoples are grouped by building uuid and only get selected once
fn make_pairs(people: Vec<&People>, rng: &mut ThreadRng) -> Vec<(People, People)> {
    let mut building_groups: HashMap<LayoutId, Vec<&People>> = HashMap::new();

    for person in people {
        building_groups
            .entry(
                person
                    .get_building_uuid()
                    .unwrap_or(&LayoutId::default())
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

pub fn update_people_in_building(engine: &LockableEngine, rng: &mut ThreadRng) {
    lock_read!(engine |> read);
    let buildings = read.layout.get_buildings();
    let (full_buildings, empty_buildings): (Vec<_>, Vec<_>) = buildings
        .iter()
        .partition(|&b| b.is_overcrowded(&read.population));

    let graph = read.layout.graph.clone();
    lock_unlock!(read);

    lock_write!(engine |> write);
    for people in write
        .population
        .get_core_district_mut()
        .peoples
        .iter_mut()
        .filter_map(|p| p.as_alive_mut())
        .filter(|p| {
            p.building_uuid.is_none()
                || full_buildings
                    .iter()
                    .any(|b| b.get_building_uuid() == p.building_uuid.unwrap())
                    && rng.random_bool(0.2)
        })
    {
        if let Some(ref g) = graph {
            let conexions = g.get_buildings_connections(people.building_uuid.unwrap());
            debug!("NUMBER of conex: {}", conexions.len());

            let target = conexions.get(0);

            if let Some(building_id) = target {
                for building in empty_buildings
                    .iter()
                    .filter(|b| b.get_building_uuid() == *building_id)
                {
                    people.building_uuid = Some(building.id);
                }
            }
            else {
                let id = empty_buildings.choose(&mut rand::rng()).unwrap().get_building_uuid();
                people.building_uuid = Some(id);
            }
        }
    }

    lock_unlock!(write);
}
