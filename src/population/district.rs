use crate::{engine::layout::BuildingType, population::*, LAYOUT};
use rand::{rng, seq::IndexedRandom};
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
    pub id: usize,
    pub peoples: Vec<People>,
    pub zone_type: DistrictZone,

    pub num_people: usize,
    /// max number of people before overpopulation.
    pub capacity: u16,

    /// maximum number of other district this district can be next to.
    pub max_num_neighbors: usize,
    pub neighbors: Vec<usize>,

    /// Use get_percentage instead
    pub num_happiness: u16,
    /// Use get_percentage instead
    pub num_sick: u16, // use
    pub working_poulation: u16,
}

impl PopulationDistrict {
    pub fn get_population_number_by(&self, find_by: PeopleLegalState) -> usize {
        self.peoples
            .iter()
            .filter(|p| p.get_legal_state() == find_by)
            .count()
    }

    pub fn recalcul_happiness(&mut self) {
        self.num_happiness = Self::aux_happiness(&self.peoples);
    }
    /// Will just add the number to the field
    pub fn update_happiness(&mut self, peoples: &Vec<People>) {
        self.num_happiness += Self::aux_happiness(peoples);
    }

    pub fn recalcul_sickness(&mut self) {
        self.num_sick = Self::aux_sickness(&self.peoples);
    }
    /// Will just add the number to the field
    pub fn update_sickness(&mut self, peoples: &Vec<People>) {
        self.num_happiness += Self::aux_sickness(peoples);
    }

    pub fn recalcul_working_population(&mut self) {
        self.working_poulation = Self::aux_working_peoples(&self.peoples);
    }
    /// Will just add the number to the field
    pub fn update_working_population(&mut self, peoples: &Vec<People>) {
        self.working_poulation += Self::aux_working_peoples(peoples);
    }

    /// Give the correct happiness percentage
    pub fn get_happiness_percentage(&self) -> f32 {
        (((self.num_people / 2) as u16 + self.num_happiness) as f32 / self.num_people as f32)
            .clamp(0f32, 1f32)
    }

    pub fn get_sick_percentage(&self) -> f32 {
        (self.num_sick as f32 / self.num_sick as f32).clamp(0f32, 1f32)
    }

    pub fn add_peoples(&mut self, peoples: &mut Vec<People>) {
        self.num_people += peoples.len();

        self.update_happiness(peoples);
        self.update_sickness(peoples);
        self.update_working_population(peoples);

        self.peoples.append(peoples);
    }

    pub fn update_building_occupation(&mut self) {
        let mut binding = LAYOUT.lock().unwrap();
        let buildings: Vec<&mut Building> = binding.get_buildings_district_mut(self.id);

        let mut rng = rng();
        for people in self
            .peoples
            .iter_mut()
            .filter(|people| people.get_legal_state() != PeopleLegalState::Child)
        {
            if let Some(people) = people.as_alive_mut() {
                people.building_uuid = Some(
                    buildings
                        .iter()
                        .filter(|b| b.get_building_type() != BuildingType::EmptySpace)
                        .collect::<Vec<_>>()
                        .choose(&mut rng)
                        .unwrap()
                        .get_building_uuid(),
                );
            }
        }
    }

    // ----- not public -----

    fn aux_happiness(peoples: &Vec<People>) -> u16 {
        let mut res = 0;
        for people in peoples {
            match people {
                People::Alive(AlivePerson { mood, .. }) => res += *mood as u16,
                People::Dead { .. } => res -= 1,
            }
        }
        res
    }

    fn aux_sickness(peoples: &Vec<People>) -> u16 {
        let mut res = 0;
        for people in peoples {
            match people {
                People::Alive(AlivePerson { disease, .. }) if { *disease != None } => res += 1,
                _ => (),
            }
        }

        res
    }

    fn aux_working_peoples(peoples: &Vec<People>) -> u16 {
        let mut res = 0;
        for people in peoples {
            match people {
                People::Alive(AlivePerson { work_status, .. }) if work_status.is_some() => res += 1,
                _ => (),
            }
        }

        res
    }
}
