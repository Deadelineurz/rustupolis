use crate::population::dna::*;
use crate::population::disease::*;

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
    Angry = -2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasePeople {
    pub age: u8,
    pub peopletype: DNA
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum People {
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