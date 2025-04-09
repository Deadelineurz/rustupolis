use deaths::check_death;
use rand::{rng, rngs::ThreadRng, seq::SliceRandom};
use spawn_child::{number_of_children_to_make, spawn_childs};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::sync::mpsc::Sender;
use crate::engine::core::{Engine, LockableEngine};
use crate::engine::layout::LayoutId;
use crate::{engine::layout::Building, lock_read, lock_unlock, lock_write, population::{
    district::PopulationDistrict,
    people::{BasePeopleInfo, People, PeopleLegalState},
    Population,
}, send_to_side_bar_auto, ui::sidebar::{LogColor, LogType}};
use crate::threads::sidebar::SideBarMessage;
use crate::utils::send_to_side_bar_read;

pub mod deaths;
pub mod dna_transmission;
pub mod spawn_child;

/// Kinda expensive, will do a DFS on the districts then shuffle the population to make babies.
pub fn update_population(engine: &LockableEngine, debug: bool) {
    lock_read!(engine |> e);
    let order = e.population.num_districts;
    let mut marks: Vec<bool> = vec![false; order];
    drop(e);

    let mut rng = rng();

    update_district_peoples(engine, 0, &mut marks, &mut rng, debug);
}

fn update_district_peoples(
    engine: &Arc<RwLock<Engine>>,
    district_id: usize,
    marked: &mut Vec<bool>,
    rng: &mut ThreadRng,
    debug: bool
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
            .for_each(|alive| alive.age += 1);

        update_births(s.clone(), district, rng, debug);
        update_deaths(s, district, debug);

        let clones = district.neighbors.clone();
        pop.refresh();
        lock_unlock!(pop);

        lock_read!(engine |> rlock);

        if debug {
            send_to_side_bar_auto!(r, &rlock, "One year has passed", LogType::City, LogColor::Normal);
        }

        for district_id in clones {
            update_district_peoples(engine, district_id, marked, rng, debug);
        }
    }
}

/// Will shuffle the district's population because of the parents
fn update_births(
    pipe: Sender<SideBarMessage>,
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
        let _ = pipe.send(SideBarMessage::Single(Box::new(format!("Births: {}", childs.len())), LogType::City, LogColor::Normal));
    }

    district.add_peoples(&mut childs);
}



fn update_deaths(pipe: Sender<SideBarMessage>, district: &mut PopulationDistrict, debug: bool) {
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
        let _ = pipe.send(SideBarMessage::Single(Box::new(format!("Deaths {}", district.get_population_number_by(PeopleLegalState::Dead) - bef)), LogType::City, LogColor::Normal));
    }
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

fn _update_people_in_building(_peoples: Vec<&People>, _rng: &mut ThreadRng, _buildings: &mut Building) {
    todo!();
}
