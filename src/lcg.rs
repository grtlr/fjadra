const A: u64 = 1_664_525;
const C: u64 = 1_013_904_223;
const M: u64 = 4_294_967_296;

/// A simple linear congruential generator (LCG) that produces a sequence of pseudo-random numbers.
///
/// By default the LCG is initialized with a seed of 0. The parameters are as follows:
/// ```rs
/// const A: u64 = 1_664_525;
/// const C: u64 = 1_013_904_223;
/// const M: u64 = 4_294_967_296;
/// ```
#[derive(Clone, Debug)]
pub struct Lcg {
    state: u64,
}

impl Lcg {
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Default for Lcg {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Iterator for Lcg {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.state = self.state.wrapping_mul(A).wrapping_add(C) % M;
        Some(self.state as f64 / M as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_same_values_as_d3() {
        // Initialize with seed 1
        let mut lcg = Lcg::new(1);

        // Known sequence generated by the parameters (a, c, m) and seed 1
        let expected_values = [
            0.23645552527159452,
            0.3692706737201661,
            0.5042420323006809,
            0.7048832636792213,
        ];

        // Check that the first 5 values generated by the LCG match the expected values
        for expected in expected_values {
            let generated = lcg.next().unwrap();
            assert_eq!(
                generated, expected,
                "Expected {expected}, but got {generated}"
            );
        }
    }

    #[test]
    fn repeatability() {
        // Initialize two LCGs with the same seed
        let mut lcg1 = Lcg::new(12345);
        let mut lcg2 = Lcg::new(12345);

        // Generate a sequence from both and check that they are identical
        for _ in 0..1000 {
            let value1 = lcg1.next().unwrap();
            let value2 = lcg2.next().unwrap();
            assert_eq!(value1, value2, "Values diverged: {value1} != {value2}");
        }
    }

    #[test]
    fn different_seeds() {
        // Initialize two LCGs with different seeds
        let mut lcg1 = Lcg::new(1);
        let mut lcg2 = Lcg::new(2);

        // Check that their sequences differ
        let mut diverged = false;
        for _ in 0..1000 {
            let value1 = lcg1.next().unwrap();
            let value2 = lcg2.next().unwrap();
            if value1 != value2 {
                diverged = true;
                break;
            }
        }

        assert!(diverged, "Sequences did not diverge for different seeds");
    }
}
