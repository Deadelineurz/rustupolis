use crate::population::{
    disease::{Disease, DiseaseLethality},
    district::{DistrictZone, PopulationDistrict},
    dna::{DNAFlags, DNA},
    people::{AlivePerson, Mood, People},
};

use super::dna_transmission::{mix_dna, mutate_dna};

const MUTATION_PERCENTAGE: f64 = 0.07;
const UNSTABLE_DNA_MUTATION_BONUS: f64 = 0.13;

/// return 0, 1, or 2 childrens to make
pub fn number_of_children_to_make(people: &AlivePerson, env: &PopulationDistrict) -> u8 {
    let birth_probability = fertility_from_age(people.get_age())
        * fertility_bonus(people.dna)
        * mood_bonus(&people.mood)
        * happiness_bonus(env.get_happiness_percentage().into())
        * zone_bonus(&env.zone_type)
        * sickness_bonus(&people.disease)
        * work_bonus(
            &people.work_status.is_some(),
            env.working_poulation as f64 / env.num_people as f64,
        );
    let base = birth_probability.floor() as u8;
    if rand::random::<f64>() < (birth_probability - base as f64) {
        base + 1
    } else {
        base
    }
}

pub fn spawn_childs(
    amount: u8,
    is_witness: bool,
    parent1: &AlivePerson,
    parent2: &AlivePerson,
) -> Vec<People> {
    let mut vec = Vec::new();
    for i in 0..(amount + if is_witness { 1 } else { 0 }) {
        let mut dna = mix_dna(parent1.dna, parent2.dna);

        let bonus_mutation = match (
            parent1.dna.contains(DNAFlags::UnstableDNA),
            parent2.dna.contains(DNAFlags::UnstableDNA),
        ) {
            (x, y) if x && y => UNSTABLE_DNA_MUTATION_BONUS * 2.0,
            (x, y) if x || y => UNSTABLE_DNA_MUTATION_BONUS,
            _ => 0.0,
        };

        dna = mutate_dna(dna, MUTATION_PERCENTAGE + bonus_mutation);

        vec.push(People::create_people(
            dna,
            parent1.mood.to_average(parent2.mood),
            parent1.building_uuid.clone(),
            i == 0 && is_witness,
        ));
    }

    vec
}

fn fertility_bonus(dna: DNA) -> f64 {
    if dna.contains(DNAFlags::HighFertility) {
        1.2
    } else if dna.contains(DNAFlags::LowFertility) {
        0.65
    } else {
        1.0
    }
}

/// Simple bell curve centred arround 30 years
fn fertility_from_age(age: u32) -> f64 {
    f64::exp(-((age as f64 - 30.0) as f64 / 10.0).powf(2.0))
}

fn happiness_bonus(percentage: f64) -> f64 {
    0.5 + 0.5 * percentage
}

fn zone_bonus(district_type: &DistrictZone) -> f64 {
    match district_type {
        DistrictZone::Slums => 1.2,
        DistrictZone::Industrials => 0.85,
        DistrictZone::Residentials => 1.1,
        DistrictZone::Core => 0.7,
    }
}

fn mood_bonus(mood: &Mood) -> f64 {
    match mood {
        Mood::Zealot => 1.15,
        Mood::Happy => 1.1,
        Mood::Neutral => 1.0,
        Mood::Unhappy => 0.8,
        Mood::Angry => 0.7,
    }
}

fn sickness_bonus(disease: &Option<Disease>) -> f64 {
    if let Some(disease) = disease {
        match disease.lethality {
            DiseaseLethality::Low => 0.9,
            DiseaseLethality::Moderate => 0.6,
            DiseaseLethality::Deadly => 0.1,
        }
    } else {
        1.0
    }
}

fn work_bonus(is_working: &bool, work_percentage: f64) -> f64 {
    (if *is_working { 0.9 } else { 1.1 }) * 1.1 - 0.2 * work_percentage
}
