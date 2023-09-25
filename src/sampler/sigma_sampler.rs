use rand::Rng;
use rand_distr::{Distribution, Gamma};

#[derive(Debug, Clone, Copy)]
pub struct SigmaSampler {
    pub nu: f64,
    pub lambda: f64,
    pub rmse: f64,
    pub scale: f64,
}

impl SigmaSampler {
    pub fn new(nu: f64, lambda: f64) -> Self {
        if nu <= 0.0 {
            panic!("nu: {} should large 0", nu);
        }

        if lambda <= 0.0 {
            panic!("lambda: {} should large 0", lambda);
        }

        Self {
            nu,
            lambda,
            rmse: f64::INFINITY,
            scale: 1.0,
        }
    }

    pub fn default() -> Self {
        Self::new(3.0, 1.0)
    }

    pub fn sample<R: Rng + ?Sized>(&mut self, resid: &Vec<f64>, rng: &mut R) -> f64 {
        let resid_sqr_sum = resid.iter().fold(0.0, |acc, r| acc + r * r);
        let n = resid.len() as f64;
        self.rmse = f64::sqrt(resid_sqr_sum / n);
        let nu = (self.nu + n) / 2.0;
        let lambda = (self.nu * self.lambda + resid_sqr_sum) / 2.0;
        let gamma = Gamma::new(nu, 1.0 / lambda).unwrap();
        1.0 / gamma.sample(rng)
    }
}
