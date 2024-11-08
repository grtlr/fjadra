use crate::lcg::Lcg;

pub fn jiggle(random_gen: &mut Lcg) -> f64 {
    match random_gen.next() {
        Some(x) => (x - 0.5) * 1e-6,
        _ => unreachable!(),
    }
}
