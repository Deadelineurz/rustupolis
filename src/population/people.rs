use crate::engine::layout::Building;
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
    Poverty,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkLethality {
    SafeJob,
    MediumRisks,
    HighRisks,
    ExtremeRisks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeopleLegalState {
    Baby,
    Child,
    Adult,
    Elder,
    Dead,
    /// this person should not be alive
    Anomaly,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlivePerson {
    pub age: u8,
    pub dna: DNA,
    pub mood: Mood,
    pub disease: Option<Disease>,
    /// If the work status is `None`, then this person has no job.
    pub work_status: Option<WorkLethality>,
    pub building_uuid: Option<String>,
    pub is_witness: bool
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadPerson {
    pub age: u8,
    pub dna: DNA,
    pub cause: CauseOfDeath,
    pub building_uuid: Option<String>, // yep, the corpse can still be in a building lol
    pub is_witness: bool
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum People {
    Alive(AlivePerson),
    Dead(DeadPerson),
}
pub trait BasePeopleInfo {
    fn get_age(&self) -> u8;
    fn get_dna(&self) -> DNA;
    fn get_building_uuid(&self) -> Option<&String>;
    fn is_witness(&self) -> bool;
}

impl BasePeopleInfo for People {
    fn get_age(&self) -> u8 {
        match self {
            People::Alive(AlivePerson { age, .. }) => *age,
            People::Dead(DeadPerson { age, .. }) => *age,
        }
    }

    fn get_dna(&self) -> DNA {
        match self {
            People::Alive(AlivePerson { dna, .. }) => *dna,
            People::Dead(DeadPerson { dna, .. }) => *dna,
        }
    }

    fn get_building_uuid(&self) -> Option<&String> {
        match self {
            People::Alive(AlivePerson { building_uuid, .. }) => building_uuid.as_ref(),
            People::Dead(DeadPerson { building_uuid, .. }) => building_uuid.as_ref(),
        }
    }

    fn is_witness(&self) -> bool {
        match self {
            People::Alive(AlivePerson { is_witness, .. }) => *is_witness,
            People::Dead(DeadPerson { is_witness, .. }) => *is_witness,
        }
    }
}

impl People {
    /// Try cast People into alive
    pub fn as_alive(&self) -> Option<&AlivePerson> {
        if let People::Alive(person) = self {
            Some(person)
        } else {
            None
        }
    }

    pub fn as_alive_mut(&mut self) -> Option<&mut AlivePerson> {
        if let People::Alive(person) = self {
            Some(person)
        } else {
            None
        }
    }

    /// Try cast People into dead
    pub fn as_dead(&self) -> Option<&DeadPerson> {
        if let People::Dead(person) = self {
            Some(person)
        } else {
            None
        }
    }

    /// If the person is an adult/child/baby...
    pub fn get_legal_state(&self) -> PeopleLegalState {
        if let People::Alive(AlivePerson { age, .. }) = &self {
            match age {
                age if *age < 4 => PeopleLegalState::Baby,
                age if *age < 18 => PeopleLegalState::Child,
                age if *age < 64 => PeopleLegalState::Adult,
                age if *age < 110 => PeopleLegalState::Elder,
                _ => PeopleLegalState::Anomaly,
            }
        } else {
            PeopleLegalState::Dead
        }
    }

    pub fn is_same_building(&self, people: People) -> bool {
        self.get_building_uuid() == people.get_building_uuid()
    }

    /// Effectively kill the person.
    pub fn make_dead(&mut self, cause_of_death: CauseOfDeath) {
        *self = People::Dead(DeadPerson {
            age: self.get_age(),
            dna: self.get_dna(),
            cause: cause_of_death,
            building_uuid: self.get_building_uuid().cloned(),
            is_witness: false
        })
    }

    /// A BLOODY MIRACLE! \
    /// Will not work if the person is still alive
    pub fn ressurect(&mut self) {
        if self.as_alive() != None {
            return;
        }

        *self = People::Alive(AlivePerson {
            age: self.get_age(),
            dna: self.get_dna(),
            building_uuid: self.get_building_uuid().cloned(),
            mood: Mood::Neutral,
            disease: None,
            work_status: None,
            is_witness: false
        })
    }

    /// Create a new alive people, will add a random number of DNA traits
    pub fn create_random_people(working_age: bool, max_dna_traits: u8) -> Self {
        let age = match working_age {
            true => 18 + rand::random_range(0..=32),
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
            work_status: None,
            building_uuid: None,
            is_witness: false
        })
    }

        /// Create a new alive people, which history will be logged
        pub fn create_witness(age: u8, dna_traits: u32, starting_building: Option<Building>) -> Self {

            People::Alive(AlivePerson {
                age,
                dna: DNA::from_flag(dna_traits),
                mood: Mood::Neutral,
                disease: None,
                work_status: None,
                building_uuid: if let Some(building) = starting_building { Some(building.get_building_uuid()) } else { None },
                is_witness: true
            })
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
