use crate::population::disease::*;
use crate::population::dna::*;
use rand::random_range;
use strum::EnumCount;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CauseOfDeath {
    OldAge,
    Murder,
    Sickness,
    Radiations,
    WorkAccident,
    EatenByMonster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Zealot = 2, // see the Mayor as their god and so won't notice the state they are in.
    Happy = 1,
    Neutral = 0,
    Unhappy = -1,
    Angry = -2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasePeople {
    pub age: u8,
    pub dna: DNA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum People {
    Alive {
        base: BasePeople,
        mood: Mood,
        disease: Option<Disease>,
        is_working: bool
    },
    Dead {
        base: BasePeople,
        cause: CauseOfDeath,
    },
}

impl People {
    /// Create a new alive people, will add a random number of DNA traits
    pub fn create_random_people(working_age: bool, max_dna_traits: u8) -> Self {
        let age = match working_age {
            true => 18 + rand::random_range(0..=17),
            false => rand::random_range(0..18),
        };

        let mut dna_traits: u32 = 0;
        for _ in 0..max_dna_traits {
            dna_traits |= 2_u32.pow(random_range(0_u32..DNAFlags::COUNT as u32));
        }

        People::Alive {
            base: BasePeople {
                age,
                dna: DNA::from_flag(dna_traits),
            },
            mood: Mood::Neutral,
            disease: None,
            is_working: false
        }
    }

    /// Create new (alive) peoples with 1 DNA trait each.
    pub fn create_random_population(amount : u8) -> Vec<People> {
        let mut peoples = vec![];
        for _ in 0..amount {
            peoples.push(People::create_random_people(true, 1));
        }

        peoples
    }
}
