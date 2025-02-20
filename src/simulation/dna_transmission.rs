use crate::population::dna::DNA;

pub fn mix_dna(parent1: DNA, parent2: DNA) -> DNA {
    let mut child_dna = DNA::new();
    for i in 0..32 {
        let mask = 1 << i;
        let bit1 = parent1 & DNA::from_flag(mask);
        let bit2 = parent2 & DNA::from_flag(mask);

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

pub fn mutate_dna(dna: DNA, mutation_rate: f64) -> DNA {
    let mut mutated_dna = dna;
    for i in 0..32 {
        if rand::random::<f64>() < mutation_rate {
            mutated_dna ^= DNA::from_flag(1 << i);
        }
    }
    mutated_dna
}
