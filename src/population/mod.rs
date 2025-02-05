use std::ops::{BitAnd, BitOr, BitXor, Not};

pub enum CauseOfDeath {
    OldAge,
    Murder,
    Sickness,
    Radiations,
    WorkAccident,
    EatenByMonster,
}

pub enum Mood {
    Zealot, // see the Mayor as their god and so won't notice the state they are in.
    Happy,
    Neutral,
    Unhappy,
}

pub struct BasePeople {
    pub age: u8,
    pub peopletype: PeopleDNA,
}

enum People {
    Alive {
        base: BasePeople,
        mood: Mood,
        disease: Option<Disease>,
    },
    Dead {
        base: BasePeople,
        cause: CauseOfDeath,
    },
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PeopleDNA {
    None =              0,
    Irradiated =        1,  // Will live less
    Fragile =           2,  // More likely to catch a desease


    Immortal =          4,  // Can not die of injury
    Unaging =           8,  // Can not age
    Immune =            16, // Can not become sick 
    HealthyCarrier =    32, // Can NOT die from his own desease, but can spread it
}

impl BitAnd for PeopleDNA {
    type Output = u8;

    fn bitand(self, rhs: Self) -> Self::Output {
        self as u8 & rhs as u8
    }
}

impl BitOr for PeopleDNA {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitXor for PeopleDNA {
    type Output = u8;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self as u8 ^ rhs as u8
    }
}

impl Not for PeopleDNA {
    type Output = u8;

    fn not(self) -> Self::Output {
        !(self as u8)
    }
}

// _____ Desease _____

#[derive(Debug, Clone, Copy)]
pub enum DiseaseName {
    Bacteria, // should probably be real name and not diseases type
              //...
}

#[derive(Debug, Clone, Copy)]
pub enum DiseaseLethality {
    Deadly,
    Moderate,
    Low,
}

#[derive(Debug, Clone, Copy)]
pub enum DiseaseContagionRate {
    High,
    Mederate,
    Low,
}

#[derive(Debug, Clone, Copy)]
struct Disease {
    pub name: DiseaseName,
    pub lethality: DiseaseLethality,
    pub contagion: DiseaseContagionRate,
}
