// Pour chaque district
//  1. Mise à jour de la population
//      - création d'enfant
//          - prendre en compte âge
//          - prendre en compte âge

fn mix_dna(parent1: u32, parent2: u32) -> u32 {
    let mut child_dna = 0;
    for i in 0..32 {
        let mask = 1 << i;
        let bit1 = parent1 & mask;
        let bit2 = parent2 & mask;

        if bit1 == bit2 {
            child_dna |= bit1;
        } else {
            if rand::random::<f64>() < 0.7 {
                child_dna |= bit1.max(bit2);
            } else {
                child_dna |= bit1.min(bit2);
            }
        }
    }
    child_dna
}
