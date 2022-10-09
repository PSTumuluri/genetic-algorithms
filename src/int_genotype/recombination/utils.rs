/// Utility function that returns the child constructed by taking the first
/// part of one genotype with the second part of another genotype.
/// DOES NOT CHECK THE CUTOFF POINT OR PARENT COMPATIBILITY BECAUSE THESE
/// SHOULD HAVE ALREADY BEEN CHECKED BY THE OPERATOR CALLING THIS FUNCTION.
pub fn crossover_at_point(parent1: &[i64], parent2: &[i64], pivot: usize)
-> Vec<i64> {
    parent1[..pivot]
        .iter()
        .chain(parent2[pivot..].iter())
        .copied()
        .collect::<Vec<i64>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossover_at_point_works() {
        let parent1 = [1, 2, 3, 4];
        let parent2 = [4, 3, 2, 1];
        let child = crossover_at_point(&parent1, &parent2, 2);
        assert_eq!(vec![1, 2, 2, 1], child);

        let child = crossover_at_point(&parent1, &parent2, 3);
        assert_eq!(vec![1, 2, 3, 1], child);
    }
}
