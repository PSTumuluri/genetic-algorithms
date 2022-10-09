use super::*;

mod utils;

/// Currently uses a constant value rather than randomly sampling.
/// TODO: Find a way to randomly sample integers from a distribution that
/// is both symmetric about 0 and has a higher probability of sampling
/// values closer to 0.
fn creep_mutation(genotype: &mut [i64], range: Range<i64>, creep: i64, 
                  freq: &OperatorFreq, rng: &mut impl Rng) {
    let freq = freq.value();
    for allele in genotype.iter_mut() {
        if rng.gen::<f64>() < freq {
            let positive = rng.gen::<bool>();
            if positive {
                *allele = utils::range_saturating_add(*allele, creep, &range);
            } else {
                *allele = utils::range_saturating_add(*allele, -creep, &range);
            }
        }
    }
}

fn random_mutation(genotype: &mut [i64], range: &Range<i64>,
                   freq: &OperatorFreq, rng: &mut impl Rng) {
    let freq = freq.value();
    let distr = Uniform::from(range.clone());
    for allele in genotype.iter_mut() {
        if rng.gen::<f64>() < freq {
            *allele = distr.sample(rng);
        }
    }
}
