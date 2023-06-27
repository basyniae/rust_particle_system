use rand::distributions::{Distribution, Standard};
use rand::Rng;

/// Struct equipped with standard exponential distribution (rate parameter 1).
/// # Example
/// `let x: StandardExponential = rng.gen()`
/// Then x.0 will have standard the standard exponential distribution.
#[derive(Debug)]
pub struct StandardExponential(pub f64);

impl Distribution<StandardExponential> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StandardExponential {
        let uniform: f64 = rng.gen_range(0.0..1.0);
        StandardExponential(-uniform.ln())
    }
}