pub mod disease;
pub mod district;
pub mod dna;
pub mod people;

use district::*;
use people::*;

#[derive(Debug)]
pub struct Population {
    pub num_slice: u8,
    districts: Vec<PopulationDistrict>,
}

impl Population {
    /// Will also add an empty Core district.
    pub fn new() -> Population {
        let mut pop = Population {
            num_slice: 0,
            districts: Vec::new(),
        };

        PopulationDistrict::instantiate(Vec::new(), DistrictZone::Core, &mut pop);

        pop
    }

    pub fn get_district(&self, id: u8) -> &PopulationDistrict {
        &self.districts[id as usize]
    }

    pub fn remove_slice_by_id(&mut self, id: u8) {
        self.districts.remove(id.into());
        self.num_slice -= 1;
    }

    /// Be sure that the Population contains at least one district (which *should* be a core district).
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

    pub fn get_districts_by_type(&self, zone: DistrictZone) -> Vec<&PopulationDistrict> {
        self.districts
            .iter()
            .filter(|district| district.zone_type == zone)
            .collect() // collect my beloved
    }
}
