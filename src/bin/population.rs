use rustupolis::population::*;
// use rustupolis::population::district::*;
use rustupolis::simulation::update_population;

fn main() {
    // By default a popluation start with a core district.
    let mut population = Population::new();

    // add 50 peoples to Core district. (by default (None), it is the Core district)
    population.add_peoples(50, None);

    // // add 80 people to the newly created slums
    // let slum_id = population.add_district(80, DistrictZone::Slums);
    for i in 0..100 {
        println!("_________________Year {i}____________________");
        dbg!(&population.get_core_district().peoples.len());

        update_population(&mut population);

        dbg!(&population.get_core_district().peoples.len());
    }
}
