use rustupolis::population::dna::*;
use rustupolis::population::people::*;
use rustupolis::population::*;
use rustupolis::population::district::*;

fn main() {
    let kevin = People::Alive {
        base: BasePeople {
            age: 30,
            peopletype: DNA::from_flags(DNAFlags::StrongPhysique | DNAFlags::Aggressive),
        },
        mood: Mood::Neutral,
        disease: None,
    };

    dbg!(kevin);

    let mut population = Population::new();

    let _district =
        PopulationDistrict::instantiate(vec![kevin], DistrictZone::Residentials, &mut population);
}
