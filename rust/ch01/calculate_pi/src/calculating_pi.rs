
pub fn calculate_pi(n_terms: u32) -> f64 {
  let numerator: f64 = 4.0;
  let mut denominator: f64 = 1.0;
  let mut operation: f64 = 1.0;

  let mut pi: f64 = 0.0;

  for _ in 0..n_terms {
    pi += operation * (numerator / denominator);
    denominator += 2.0;
    operation *= -1.0;
  }

  pi
}