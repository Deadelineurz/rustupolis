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

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Zealot = 2, // see the Mayor as their god and so won't notice the state they are in.
    Happy = 1,
    Neutral = 0,
    Unhappy = -1,
    Angry = -2,
}

impl Mood {
    /// return (mood1 + mood2) / 2 as new mood.
    pub fn to_average(&self, mood: Mood) -> Self {
        match (*self as i8 + mood as i8) / 2 {
            2 => Mood::Zealot,
            1 => Mood::Happy,
            -1 => Mood::Unhappy,
            -2 => Mood::Angry,
            _ => Mood::Neutral,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlivePerson {
    pub age: u8,
    pub dna: DNA,
    pub mood: Mood,
    pub disease: Option<Disease>,
    pub is_working: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeadPerson {
    pub age: u8,
    pub dna: DNA,
    cause: CauseOfDeath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum People {
    Alive(AlivePerson),
    Dead(DeadPerson),
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

        People::Alive(AlivePerson {
            age,
            dna: DNA::from_flag(dna_traits),
            mood: Mood::Neutral,
            disease: None,
            is_working: false,
        })
    }

    fn as_alive(&self) -> Option<&AlivePerson> {
        if let People::Alive(person) = self {
            Some(person)
        } else {
            None
        }
    }

    fn as_dead(&self) -> Option<&DeadPerson> {
        if let People::Dead(person) = self {
            Some(person)
        } else {
            None
        }
    }

    /// Create new (alive) peoples with 1 DNA trait each.
    pub fn create_random_population(amount: u8) -> Vec<People> {
        let mut peoples = vec![];
        for _ in 0..amount {
            peoples.push(People::create_random_people(true, 1));
        }

        peoples
    }
}
