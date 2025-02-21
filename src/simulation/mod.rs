use std::collections::HashMap;

use deaths::check_death;
use rand::{rng, rngs::ThreadRng, seq::SliceRandom};
use spawn_child::{number_of_children_to_make, spawn_childs};

use crate::population::{
    district::PopulationDistrict,
    people::{BasePeopleInfo, People},
    Population,
};

pub mod deaths;
pub mod dna_transmission;
pub mod spawn_child;

/// Kinda expensive, will do a DFS on the districts then shuffle the population to make babies.
pub fn update_population(population: &mut Population) {
    let order = population.num_districts;
    let mut marks: Vec<bool> = vec![false; order];

    let mut rng = rng();

    update_district_peoples(0, population, &mut marks, &mut rng);
}

fn update_district_peoples(
    district_id: usize,
    population: &mut Population,
    marked: &mut Vec<bool>,
    rng: &mut ThreadRng,
) {
    if !marked[district_id] {
        marked[district_id] = true;

        let district = population.get_district_mut(district_id).unwrap();

        update_births(district, rng);
        update_deaths(district);

        for district_id in district.neighbors.clone() {
            update_district_peoples(district_id, population, marked, rng);
        }
    }
}

/// Will shuffle the district's population because of the parents
fn update_births(district: &mut PopulationDistrict, rng: &mut ThreadRng) {
    let childs: Vec<People> = make_pairs(&district.peoples, rng)
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

    district.peoples.extend(childs);
}

fn update_deaths(district: &mut PopulationDistrict) {
    let zone = district.zone_type.clone();
    let happiness: f64 = district.happiness_percentage().into();


    let _deaths = district
        .peoples
        .iter_mut()
        .filter(|people| people.as_alive() != None)
        .map(|people| 
            if let Some(cause) = check_death(people.as_alive().unwrap(), zone, happiness) {
                people.make_dead(cause);
            }
        );
}

/// Ensure that the peoples are grouped by building uuid and only get selected once
fn make_pairs(people: &Vec<People>, rng: &mut ThreadRng) -> Vec<(People, People)> {
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
