use crate::population::*;
use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum DistrictZone {
    Core,
    Residentials,
    Slums,
    Industrials,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopulationDistrict {
    pub peoples: Vec<People>,
    pub zone_type: DistrictZone,

    pub num_people: usize,
    /// max number of people before overpopulation.
    pub capacity: u16,

    /// maximum number of other district this district can be next to.
    pub max_num_neighbors: usize,
    pub neighbors: Vec<usize>,
    
    /// if you want the real number, you should add half the population
    pub num_happiness: u16,
    pub num_sick: u16,      // use get_percentage
    pub working_poulation: u16,
}

impl PopulationDistrict {

    fn get_happiness(peoples: &Vec<People>) -> u16 {
        let mut res = 0;
        for people in peoples {
            match people {
                People::Alive (AlivePerson { mood, .. }) => res += *mood as u16,
                People::Dead { .. } => res -= 1,
            }
        }
        res
    }

    fn get_sickness(peoples: &Vec<People>) -> u16 {
        let mut res = 0;
        for people in peoples {
            match people {
                People::Alive (AlivePerson{ disease, .. }) if {*disease != None} => res += 1,
                _ => (),
            }
        }

        res
    }

    fn get_working_peoples(peoples: &Vec<People>) -> u16 {
        let mut res = 0;
        for people in peoples {
            match people {
                People::Alive (AlivePerson{ work_status, .. }) if work_status.is_some() => {
                    res += 1
                }
                _ => (),
            }
        }

        res
    }

    pub fn recalcul_happiness(&mut self) {
        self.num_happiness = Self::get_happiness(&self.peoples);
    }
    /// Will just add the number to the field
    pub fn update_happiness(&mut self, peoples: &Vec<People>) {
        self.num_happiness += Self::get_happiness(peoples);
    }

    pub fn recalcul_sickness(&mut self) {
        self.num_sick = Self::get_sickness(&self.peoples);
    }
    /// Will just add the number to the field
    pub fn update_sickness(&mut self, peoples: &Vec<People>) {
        self.num_happiness += Self::get_sickness(peoples);
    }

    pub fn recalcul_working_population(&mut self) {
        self.working_poulation = Self::get_working_peoples(&self.peoples);
    }
    /// Will just add the number to the field
    pub fn update_working_population(&mut self, peoples: &Vec<People>) {
        self.working_poulation += Self::get_working_peoples(peoples);
    }

    /// Give the correct happiness percentage
    pub fn happiness_percentage(&self) -> f32 {
        (((self.num_people / 2) as u16 + self.num_happiness) as f32 / self.num_people as f32).clamp(0f32, 1f32)
    }

    pub fn sick_percentage(&self) -> f32 {
        (self.num_sick as f32 / self.num_sick as f32).clamp(0f32, 1f32)
    }

    pub fn add_peoples(&mut self, peoples: &mut Vec<People>) {
        self.num_people += peoples.len();

        self.update_happiness(peoples);
        self.update_sickness(peoples);
        self.update_working_population(peoples);

        self.peoples.append(peoples);
    }
}
