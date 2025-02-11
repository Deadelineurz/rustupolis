pub mod disease;
pub mod district;
pub mod dna;
pub mod people;

use district::*;
use people::*;

#[derive(Debug)]
pub struct Population {
    pub num_districts: u8,
    districts: Vec<PopulationDistrict>,
}

impl Population {
    /// Will also add an empty Core district.
    pub fn new() -> Population {
        let mut pop = Population {
            num_districts: 0,
            districts: Vec::new(),
        };

        pop.add_district(0, DistrictZone::Core);

        pop
    }

    // ----- GET -----

    pub fn get_district(&self, id: usize) -> &PopulationDistrict {
        &self.districts[id]
    }

    pub fn get_district_mut(&mut self, id: usize) -> &mut PopulationDistrict {
        &mut self.districts[id]
    }

    /// Should not panic.
    pub fn get_core_district(&self) -> &PopulationDistrict {
        match self
            .districts
            .iter()
            .find(|district| district.zone_type == DistrictZone::Core)
        {
            Some(x) => x,
            None => panic!("No core district found!"),
        }
    }

    /// Should not panic.
    pub fn get_core_district_mut(&mut self) -> &mut PopulationDistrict {
        match self
            .districts
            .iter_mut()
            .find(|district| district.zone_type == DistrictZone::Core)
        {
            Some(x) => x,
            None => panic!("No core district found!"),
        }
    }

    /// Return all district that connect to this district ID (read-only)
    pub fn get_district_neighbors(&self, id: usize) -> Vec<&PopulationDistrict> {
        let mut res = Vec::new();
        let district = self.get_district(id);

        for neighbor_id in &district.neighbors {
            res.push(self.get_district(*neighbor_id));
        }

        res
    }

    pub fn get_all_districts_by_zone(&self, zone: DistrictZone) -> Vec<&PopulationDistrict> {
        self.districts
            .iter()
            .filter(|district| district.zone_type == zone)
            .collect() // collect my beloved
    }

    // ----- ADD -----

    pub fn add_district(&mut self, starting_population: u8, district_zone: DistrictZone) -> usize {
        self.districts
            .push(Self::setup_district(district_zone, People::create_random_population(starting_population)));

        let index = self.districts.len() - 1;
        self.auto_link_district(index);
        index
    }

    /// If no target district provided, will add them to the Core district
    pub fn add_peoples(&mut self, amount: u16, target_district: Option<usize>) {
        let mut vec = vec![];
        for _ in 0..amount {
            vec.push(People::create_random_people(true, 1));
        }

        match target_district {
            Some(id) => self.get_district_mut(id).add_peoples(&mut vec),
            None => self.get_core_district_mut().add_peoples(&mut vec),
        }
    }

    // ----- LINKING -----

    /// Return a bool if the linking was successfull.\
    /// Probably should not do it manually ???
    pub fn link_districts(&mut self, a: usize, b: usize) -> bool {
        if a != b
            && a < self.districts.len()
            && b < self.districts.len()
            && !self.districts[a].neighbors.contains(&b)    // to not re-add the same links
            && !self.districts[b].neighbors.contains(&a)
            && self.districts[a].neighbors.len() < self.districts[a].max_num_neighbors  // to skip if we already have max_neighbors
            && self.districts[b].neighbors.len() < self.districts[b].max_num_neighbors
        {
            self.districts[a].neighbors.push(b);
            self.districts[b].neighbors.push(a);

            return true;
        }

        false
    }

    fn auto_link_district(&mut self, index: usize) {
        let max_links = self.districts[index].max_num_neighbors;

        for i in 0..index {
            if self.districts[index].neighbors.len() >= max_links {
                break;
            }
            if self.districts[i].neighbors.len() < self.districts[i].max_num_neighbors {
                self.link_districts(index, i);
            }
        }
    }

    fn setup_district(district_zone: DistrictZone, peoples: Vec<People>) -> PopulationDistrict {
        let capacity = match district_zone {
            DistrictZone::Core => 50,
            DistrictZone::Industrials => 75,
            DistrictZone::Residentials => 100,
            DistrictZone::Slums => 150,
        };

        let max_num_links = match district_zone {
            DistrictZone::Core => 4,
            DistrictZone::Industrials => 2,
            DistrictZone::Residentials => 3,
            DistrictZone::Slums => 1,
        };

        let mut district = PopulationDistrict {
            num_people: peoples.len(),
            peoples,
            zone_type: district_zone,
            neighbors: Vec::new(),
            num_happiness: 0,
            num_sick: 0,
            working_poulation: 0,
            capacity,
            max_num_neighbors: max_num_links,
        };

        district.recalcul_happiness();
        district.recalcul_sickness();
        district.recalcul_working_population();

        district
    }
}
