use super::*;

mod utils;

pub fn one_point_crossover(parent1: &[i64], parent2: &[i64], rng: &mut impl Rng)
-> Result<Vec<i64>, &'static str> {
    if parent1.len() != parent2.len() {
        return Err("recombination requires parents of compatible genotypes");
    }
    let num_genes = parent1.len();
    let pivot = rng.gen_range(1..num_genes-1);
    
    Ok(utils::crossover_at_point(parent1, parent2, pivot))
}
