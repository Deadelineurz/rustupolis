use crate::population::{
    disease::{Disease, DiseaseLethality}, district::{DistrictZone, PopulationDistrict}, dna::{DNAFlags, DNA}, people::{Mood, People}
};

/// Will panic if you give it a dead person.
pub fn number_of_children_to_make(people: &People, env: &PopulationDistrict) -> u8 {
    if let People::Alive { base, mood, disease, is_working } = people {
        let age = base.age;

        let birth_probability = 
        fertility_from_age(age)
        * fertility_bonus(base.dna)
        * mood_bonus(mood)
        * happiness_bonus(env.happiness_percentage().into())
        * zone_bonus(&env.zone_type)
        * sickness_bonus(disease)
        * work_bonus(is_working, env.working_poulation as f64 / env.num_people as f64);

        let base = birth_probability.floor() as u8;
        if rand::random::<f64>() < (birth_probability - base as f64) {
            base + 1
        } else {
            base
        }
    }
    else {
        panic!("Trying to have a dead people make babies")
    }
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
fn fertility_from_age(age: u8) -> f64 {
    f64::exp((-(age as f64 - 30.0) as f64 / 10.0).powf(2.0))
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
        Mood::Angry => 0.7
    }
}

fn sickness_bonus(disease: &Option<Disease>) -> f64 {
    if let Some(disease) = disease {
        match disease.lethality {
            DiseaseLethality::Low => 0.9,
            DiseaseLethality::Moderate => 0.6,
            DiseaseLethality::Deadly => 0.1
        }
    }
    else {
        1.0
    }
}

fn work_bonus(is_working: &bool, work_percentage: f64) -> f64 {
    (if *is_working { 0.9 } else { 1.1 }) * 1.1 - 0.2 * work_percentage
}
