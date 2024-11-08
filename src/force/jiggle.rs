use crate::lcg::Lcg;

pub fn jiggle(random_gen: &mut Lcg) -> f64 {
    (random_gen.next().unwrap() - 0.5) * 1e-6
}
