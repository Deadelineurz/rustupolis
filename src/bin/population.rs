use rustupolis::population::*;
// use rustupolis::population::district::*;
use rustupolis::simulation::update_population;

fn main() {
    // By default a popluation start with a core district.
    let mut population = Population::new();

    // add 50 peoples to Core district. (by default (None), it is the Core district)
    population.add_peoples(50, None);



    for i in 1..=50 {
        println!("_____Year {i}_____");

        println!("Before = {}\n", population.get_core_district().num_people);

        update_population(&mut population, None);

        println!("After = {}\n", population.get_core_district().num_people);
    }
}
