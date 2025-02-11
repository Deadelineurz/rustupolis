use crate::population::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistrictZone {
    Core,
    Residentials,
    Slums,
    Industrials,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopulationDistrict {
    peoples: Vec<People>,
    /// unique ID of the district in a population.
    id: u8,
    pub zone_type: DistrictZone,
    pub num_people: u16,
    num_happiness: u16, // use get_percentage
    num_sick: u16,      // use get_percentage
    pub working_poulation: u16,
    /// max number of people before overpopulation.
    pub capacity: u16,
    adj_lists: Vec<u8>,
    /// maximum number of other district this district can be next to.
    max_num_links: usize,
}

impl PopulationDistrict {
    fn update_happiness(&mut self) {
        for people in &self.peoples {
            match people {
                People::Alive { mood, .. } => self.num_happiness += *mood as u16,
                People::Dead { .. } => self.num_happiness -= 1,
            }
        }
    }

    fn update_sickness(&mut self) {
        for people in &self.peoples {
            match people {
                People::Alive { .. } => self.num_sick += 1,
                _ => (),
            }
        }
    }

    fn update_working(&mut self) {
        for people in &self.peoples {
            match people {
                People::Alive { base, .. } if base.age > 14 && base.age < 70 => {
                    self.working_poulation += 1
                }
                _ => (),
            }
        }
    }

    fn update_links(&mut self, parent_population: &mut Population) {
        let districts = &mut parent_population.districts;

        if self.adj_lists.len() >= self.max_num_links {
            return;
        }

        let self_id = self.id;

        for other in districts {
            let other_id = other.id;

            if self_id != other_id
                && !self.adj_lists.contains(&other_id)
                && other.adj_lists.len() < other.max_num_links
            {
                self.adj_lists.push(other_id);
                other.adj_lists.push(self_id);
            }
        }
    }

    /// Will add the newly created PopulationDistrict to the parent Population and init all the fields.
    pub fn instantiate(
        peoples: Vec<People>,
        zone_type: DistrictZone,
        parent_population: &mut Population,
    ) {
        let capacity = match zone_type {
            DistrictZone::Core => 50,
            DistrictZone::Industrials => 75,
            DistrictZone::Residentials => 100,
            DistrictZone::Slums => 150,
        };

        let max_num_links = match zone_type {
            DistrictZone::Core => 4,
            DistrictZone::Industrials => 2,
            DistrictZone::Residentials => 3,
            DistrictZone::Slums => 1,
        };

        let mut district = PopulationDistrict {
            num_people: peoples.len() as u16,
            peoples,
            zone_type,
            id: parent_population.num_slice,
            adj_lists: Vec::new(),
            num_happiness: 0,
            num_sick: 0,
            working_poulation: 0,
            capacity,
            max_num_links,
        };

        district.update_happiness();
        district.update_sickness();
        district.update_working();
        district.update_links(parent_population);

        parent_population.num_slice += 1;
        parent_population.districts.push(district);
    }

    pub fn happiness_percentage(&self) -> f32 {
        (self.num_happiness as f32 / self.num_people as f32).clamp(0f32, 1f32)
    }

    pub fn sick_percentage(&self) -> f32 {
        (self.num_sick as f32 / self.num_sick as f32).clamp(0f32, 1f32)
    }
}
