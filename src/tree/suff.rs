use rand::Rng;
use rand_distr::{Distribution, Normal};
use nohash::IntMap as HashMap;
// use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Suff {
    n: f64,
    wn: f64,
    sy: f64,
    old_mu: f64,
    new_mu: f64,
    diff: f64,
}

impl Suff {
    #[inline]
    pub fn default() -> Self {
        Suff {
            n: 0.0,
            wn: 0.0,
            sy: 0.0,
            old_mu: 0.0,
            new_mu: 0.0,
            diff: 0.0,
        }
    }

    #[inline]
    pub fn set_old_mu(&mut self, old_mu: f64) {
        self.old_mu = old_mu;
        self.diff = self.new_mu - self.old_mu;
    }

    #[inline]
    pub fn set_new_mu(&mut self, new_mu: f64) {
        self.new_mu = new_mu;
        self.diff = new_mu - self.old_mu;
    }

    #[inline]
    pub fn add_old_mu_to_sy(&mut self) {
        self.sy += self.wn * self.old_mu;
    }

    #[inline]
    pub fn log_likelihood(wn: f64, sy: f64, sigma: f64, tau: f64) -> f64 {
        let sigma_sqr = sigma * sigma;
        let tau_sqr = tau * tau;
        let k = sigma_sqr + tau_sqr * wn;
        f64::ln(sigma) - f64::ln(k) / 2.0 + ((tau_sqr / 2.0 / sigma_sqr) * sy * sy) / k
    }

    #[inline]
    pub fn llh(&self, sigma: f64, tau: f64, n_eta: f64) -> f64 {
        if self.wn <= n_eta {
            return f64::NEG_INFINITY;
        }
        Self::log_likelihood(self.wn, self.sy, sigma, tau)
    }

    #[inline]
    pub fn sample_mu<R: Rng + ?Sized>(&self, sigma: f64, tau: f64, rng: &mut R) -> f64 {
        let sigma_sqr = sigma * sigma;
        let tau_sqr = tau * tau;
        let posterior_var = 1.0 / (self.wn / sigma_sqr + 1.0 / tau_sqr);
        let posterior_mean = (self.sy / sigma_sqr) * posterior_var;
        Normal::new(posterior_mean, f64::sqrt(posterior_var))
            .unwrap()
            .sample(rng)
    }

    #[inline]
    pub fn update(&mut self, idx: usize, resid: &Vec<f64>, w: &Option<Vec<f64>>) {
        let w = match w {
            Some(w) => w[idx],
            None => 1.0,
        };
        self.n += 1.0;
        self.wn += w;
        self.sy += w * resid[idx];
    }

    // #[inline]
    // pub fn update_r(&mut self, r: f64, w: f64) {
    //     // let w = match w {
    //     //     Some(w) => w[idx],
    //     //     None => 1.0,
    //     // };
    //     self.n += 1.0;
    //     self.wn += w;
    //     self.sy += w * r;
    //     // self.wn += 1.0;
    //     // self.sy += resid[idx];
    // }

    // #[inline]
    // pub fn update_r_no_w(&mut self, r: f64) {
    //     // let w = match w {
    //     //     Some(w) => w[idx],
    //     //     None => 1.0,
    //     // };
    //     self.n += 1.0;
    //     self.wn += 1.0;
    //     self.sy += r;
    //     // self.wn += 1.0;
    //     // self.sy += resid[idx];
    // }


    #[inline]
    pub fn merge(l: &Self, r: &Self) -> Self {
        Suff {
            n: l.n + r.n,
            wn: l.wn + r.wn,
            sy: l.sy + r.sy,
            old_mu: l.old_mu,
            new_mu: l.new_mu,
            diff: l.diff,
        }
    }

    #[inline]
    pub fn merge_inplace(&mut self, r: &Self) {
        // Suff {
        self.n += r.n;
        self.wn += r.wn;
        self.sy += r.sy;
        // old_mu: l.old_mu,
        // new_mu: l.new_mu,
        // diff: l.diff,
        // }
        // self
    }

    #[inline]
    pub fn is_lg_n_eta(&self, n_eta: f64) -> bool {
        if self.n == 0.0 {
            self.wn > n_eta
        } else {
            self.n > n_eta
        }
    }

    #[inline]
    pub fn diff(&self) -> f64 {
        self.diff
    }

    pub fn fmt_all_suff(all_suff: &HashMap<u8, Self>) -> String {
        let mut s: Vec<String> = all_suff
            .iter()
            .map(|(k, v)| format!("node_idx:{:3}, {:?}", k, v))
            .collect();
        s.sort();
        format!("\n{}", s.join("\n"))
    }
}
