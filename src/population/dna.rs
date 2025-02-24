use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCount)]
pub enum DNAFlags {
    HealthyCarrier = 1 << 0, // can have a disease but cannot die from it
    FastMetabolism = 1 << 1, // more food but age faster
    SlowMetabolism = 1 << 2, // ^ oposite
    DiseaseResistant = 1 << 3,
    ProneToDisease = 1 << 4,
    StrongImmune = 1 << 5,
    WeakImmune = 1 << 6,
    HighFertility = 1 << 7,
    LowFertility = 1 << 8,
    Longevity = 1 << 9,
    ShortLifespan = 1 << 10,
    StrongPhysique = 1 << 11,  // Higher chance of surviving injuries
    FragilePhysique = 1 << 12, // ^ oposite
    KeenSenses = 1 << 13,      // Better at finding food or avoiding monsters
    PoorSenses = 1 << 14,      // ^ oposite
    ColdResistant = 1 << 15,
    HeatResistant = 1 << 16,
    Aggressive = 1 << 17,
    Docile = 1 << 18,
    UnstableDNA = 1 << 19,      // MORE MUTATIONS
}

impl BitAnd for DNAFlags {
    type Output = u32;

    fn bitand(self, rhs: Self) -> Self::Output {
        self as u32 & rhs as u32
    }
}

impl BitOr for DNAFlags {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl BitXor for DNAFlags {
    type Output = u32;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self as u32 ^ rhs as u32
    }
}

impl Not for DNAFlags {
    type Output = u32;

    fn not(self) -> Self::Output {
        !(self as u32)
    }
}

impl From<DNAFlags> for u32 {
    fn from(value: DNAFlags) -> Self {
        value as u32
    }
}

// the wrapper
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DNA {
    traits: u32,
}

impl DNA {
    pub fn new() -> Self {
        DNA { traits: 0 }
    }

    pub fn from_flag(flags: u32) -> Self {
        DNA {
            traits: flags,
        }
    }

    pub fn add_flag(&mut self, flag: u32) {
        self.traits |= flag as u32;
    }

    pub fn remove(&mut self, flags: DNAFlags) {
        self.traits &= !(flags as u32);
    }

    pub fn contains(&self, flags: DNAFlags) -> bool {
        (self.traits & flags as u32) != 0
    }
}

impl BitOrAssign for DNA {
    fn bitor_assign(&mut self, rhs: Self) {
        self.traits |= rhs.traits
    }
}

impl BitAndAssign for DNA {
    fn bitand_assign(&mut self, rhs: Self) {
        self.traits &= rhs.traits
    }
}

impl BitXorAssign for DNA {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.traits ^= rhs.traits
    }
}

impl BitAnd for DNA {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        DNA {
            traits: self.traits & rhs.traits
        }
    }
}

impl BitOr for DNA {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        DNA {
            traits: self.traits | rhs.traits
        }
    }
    
}

impl BitXor for DNA {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        DNA {
            traits: self.traits ^ rhs.traits
        }
    }
}

impl Not for DNA {
    type Output = Self;

    fn not(self) -> Self::Output {
        DNA {
            traits: !self.traits
        }
    }
}

// pretty debug
impl fmt::Debug for DNA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let active_traits: Vec<String> = DNAFlags::iter()
            .filter(|&flag| self.contains(flag))
            .map(|flag| format!("{:?}", flag))
            .collect(); // collect my beloved

        write!(f, "{}", active_traits.join(" | "))
    }
}
