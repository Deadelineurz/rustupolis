use rustupolis::population::dna::*;
use rustupolis::population::*;

fn main() {
    let kevin = People::Alive {
        base: BasePeople {
            age: 30,
            peopletype: DNA::from_flags(DNAFlags::StrongPhysique | DNAFlags::Aggressive)
        },
        mood: Mood::Neutral,
        disease: None,
    };

    dbg!(kevin);
}
