use rustupolis::population::*;
use rustupolis::population::district::*;

fn main() {
    // By default a popluation start with a core district.
    let mut population = Population::new();
    
    // add 50 peoples to Core district. (by default (None), it is the Core district)
    population.add_peoples(50, None); 
    
    // add 80 people to the newly created slums
    let slum_id = population.add_district(80, DistrictZone::Slums); 


    let slums_neighbors = population.get_district_neighbors(slum_id).unwrap();
    
    // When a district is added, links will be automatically added between the new and old districts
    let core_district = slums_neighbors[0];

    dbg!(&core_district.peoples[0]);
    println!("Core happiness level : {}%", core_district.happiness_percentage() * 100.0);
}
