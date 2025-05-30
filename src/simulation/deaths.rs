use rand::{rngs::ThreadRng, Rng};
use crate::engine::layout::LayoutId;
use crate::population::{
    disease::{Disease, DiseaseLethality},
    district::DistrictZone,
    dna::{DNAFlags, DNA},
    people::{AlivePerson, CauseOfDeath, Mood, WorkLethality},
};

/// Return a cause of dead if the person should *die*.
pub fn check_death(
    people: &AlivePerson,
    district_zone: DistrictZone,
    district_happiness: f64,
    rng: &mut ThreadRng
) -> Option<(CauseOfDeath, f64)> {
    let work_bonus = work_bonus(&people.work_status);
    let sickness_bonus = sickness_bonus(&people.disease);
    let zone_bonus = zone_bonus(district_zone);
    let homelesness_bonus = homeless_bonus(&people.building_uuid);
    let deathrate_from_age = deathrate_from_age(people.get_age(), people.dna);
    let dna_bonus = dna_bonus(people.dna);
    let mood_bonus = mood_bonus(&people.mood);
    let happiness_bonus = happiness_bonus(district_happiness);

    let death_probability = deathrate_from_age
        * dna_bonus
        * mood_bonus
        * happiness_bonus
        * zone_bonus
        * sickness_bonus
        * work_bonus
        * homelesness_bonus;

    let dice: f64 = rng.random();

    if dice < death_probability {
        let causes = vec![
            (CauseOfDeath::OldAge, deathrate_from_age * dna_bonus),
            (CauseOfDeath::Sickness, sickness_bonus),
            (CauseOfDeath::Radiations, zone_bonus),
            (CauseOfDeath::WorkAccident, work_bonus),
            (CauseOfDeath::Poverty, homelesness_bonus),
            (CauseOfDeath::Murder, 1.0 - mood_bonus),
            (CauseOfDeath::EatenByMonster, 1.0 - happiness_bonus),
        ];

        let total_weight: f64 = causes.iter().map(|(_, w)| w).sum();
        let mut roll = rng.random::<f64>() * total_weight;

        for (cause, weight) in causes {
            if roll < weight {
                return Some((cause, weight));
            }
            roll -= weight;
        }

        // Fallback (shouldn't happen)
        Some((CauseOfDeath::OldAge, 1.0))
    } else {
        None
    }
}

/// Simple bell curve centred arround 70 years
fn deathrate_from_age(age: u32, dna: DNA) -> f64 {
    f64::exp(
        -((age as f64
            - (60.0
                - if dna.contains(DNAFlags::ShortLifespan) {
                    25.0
                } else {
                    0.0
                }))
            / 15.0)
            .powf(2.0),
    )
}

fn happiness_bonus(percentage: f64) -> f64 {
    match percentage {
        p if p >= 0.9 => 0.9,
        p if p >= 0.7 => 1.0,
        p if p >= 0.5 => 1.1,
        p if p >= 0.3 => 1.2,
        _ => 1.4,
    }
}

fn zone_bonus(district_type: DistrictZone) -> f64 {
    match district_type {
        DistrictZone::Slums => 1.5,
        DistrictZone::Industrials => 1.2,
        DistrictZone::Residentials => 1.0,
        DistrictZone::Core => 0.9,
    }
}

fn mood_bonus(mood: &Mood) -> f64 {
    match mood {
        Mood::Zealot => 0.8,
        Mood::Happy => 0.9,
        Mood::Neutral => 1.0,
        Mood::Unhappy => 1.1,
        Mood::Angry => 1.2,
    }
}

fn sickness_bonus(disease: &Option<Disease>) -> f64 {
    if let Some(disease) = disease {
        match disease.lethality {
            DiseaseLethality::Low => 1.1,
            DiseaseLethality::Moderate => 1.5,
            DiseaseLethality::Deadly => 1.8,
        }
    } else {
        1.0
    }
}

fn work_bonus(work_lethality: &Option<WorkLethality>) -> f64 {
    if let Some(work) = work_lethality {
        match *work {
            WorkLethality::SafeJob => 1.0,
            WorkLethality::MediumRisks => 1.2,
            WorkLethality::HighRisks => 1.4,
            WorkLethality::ExtremeRisks => 1.65,
        }
    } else {
        1.0
    }
}

fn dna_bonus(dna: DNA) -> f64 {
    match dna.contains(DNAFlags::FragilePhysique) {
        true => 1.3,
        false => 1.0,
    }
}

fn homeless_bonus(building_uuid: &Option<LayoutId>) -> f64 {
    match building_uuid {
        Some(_building) => 1.0,
        None => 1.45,
    }
}
