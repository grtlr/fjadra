use crate::lcg::LCG;

pub fn jiggle(random_gen: &mut LCG) -> f64 {
    (random_gen.next().unwrap() - 0.5) * 1e-6
}
