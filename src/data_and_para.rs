use crate::sampler::sigma_sampler::SigmaSampler;
use crate::tree::split::Split;
use log::info;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};
use statrs::distribution::ChiSquared;
use statrs::distribution::ContinuousCDF;

#[derive(Debug)]
pub struct DataAndPara {
    pub x: Vec<Vec<i16>>,
    pub y: Vec<f64>,
    pub w: Option<Vec<f64>>,

    pub n: usize,
    pub p: usize,
    pub init_splits: Vec<Split>,

    pub sigma_mu: f64,
    pub mu_mu: f64,
    pub alpha: f64,
    pub beta: f64,
    pub prob_grow: f64,
    pub prob_prune: f64,
    pub prob_change: f64,
    pub n_eta: f64,
    pub block_size: usize,
}

impl DataAndPara {
    // pub fn new(
    //     x: Vec<Vec<i16>>,
    //     y: Vec<f64>,
    //     w: Option<Vec<f64>>,
    //     init_splits: Vec<Split>,
    //     sigma_mu: f64,
    //     mu_mu: f64,
    //     alpha: f64,
    //     beta: f64,
    //     prob_grow: f64,
    //     prob_prune: f64,
    //     prob_change: f64,
    //     n_eta: f64,
    // ) {
    //     let p = x.len();
    //     if p == 0 {
    //         panic!("x is empty")
    //     };

    //     if p != init_splits.len() {
    //         panic!("x.len() != init_splits.len()")
    //     };

    //     let n = y.len();
    //     let (y_sd, y_min, y_max) = Self::get_stat(&y);

    //     x.iter().for_each(|xi| {
    //         if xi.len() != n {
    //             panic!("x{} len != y.len{}", xi.len(), y.len())
    //         }
    //     });
    //     if let Some(ref w) = w {
    //         if w.len() != n {
    //             panic!("w len{} != y.len{}", w.len(), y.len())
    //         }
    //         w.iter().enumerate().for_each(|(i, &iw)| {
    //             if iw < 0.0 || f64::is_infinite(iw) || f64::is_nan(iw) {
    //                 panic!("w has unnormal value, i:{}, w:{}", i, iw)
    //             }
    //         });
    //     }
    // }

    // pub fn
    pub fn prob_split(&self, d: u8) -> f64 {
        self.alpha / f64::powf(1.0 + (d as f64), self.beta)
    }
    // pi.alpha/pow(1.0 + dnx,pi.mybeta)

    pub fn prob_not_split(&self, d: u8) -> f64 {
        1.0 - self.prob_split(d)
    }

    #[allow(dead_code)]
    fn get_stat(y: &Vec<f64>) -> (f64, f64, f64) {
        let [ys, y2s, ymin, ymax] = y
            .iter()
            .fold(
                &mut [0.0, 0.0, f64::INFINITY, f64::NEG_INFINITY],
                |acc, y| {
                    if f64::is_infinite(*y) || f64::is_nan(*y) {
                        panic!("y has unnormal value, y:{}", *y);
                    }
                    acc[0] += y;
                    acc[1] += y * y;
                    acc[2] = f64::min(acc[2], *y);
                    acc[3] = f64::max(acc[3], *y);
                    acc
                },
            )
            .clone();
        let n = y.len() as f64;
        (f64::sqrt((y2s - (ys * ys) / n) / n), ymin, ymax)
    }

    pub fn test_data(n: usize, p: usize) -> (DataAndPara, SigmaSampler, f64, f64) {
        let mut rng = thread_rng();
        let x: Vec<Vec<i16>> = (0..p)
            .map(|_i| {
                (0..n)
                    .map(|_j| rng.gen_range(0_i16..i16::MAX))
                    .collect::<Vec<i16>>()
            })
            .collect();
        let x_max = i16::MAX as f64 - 1.0;

        let y: Vec<f64> = (0..n)
            .map(|i| {
                let x0 = x[0][i] as f64 / x_max;
                let x1 = x[1][i] as f64 / x_max;
                let x2 = x[2][i] as f64 / x_max;
                let x3 = x[3][i] as f64 / x_max;
                let x4 = x[4][i] as f64 / x_max;
                let normal = Normal::new(0.0, 1.0).unwrap();
                let y = 10.0 * f64::sin(std::f64::consts::PI * x0 * x1)
                    + 20.0 * (x2 - 0.5) * (x2 - 0.5)
                    + 10.0 * x3
                    + 5.0 * x4
                    + normal.sample(&mut rng);
                y
            })
            .collect();
        let (sd, y_min, y_max) = Self::get_stat(&y);
        let (nu, q) = (3.0, 0.9);
        let chisq = ChiSquared::new(nu).unwrap();
        let qchi = chisq.inverse_cdf(1.0 - q);
        let lambda = sd * sd / qchi / nu;

        info!(
            "sd:{}, y_min:{}, y_max:{}, nu:{}, q:{}, lambda:{}",
            sd, y_min, y_max, nu, q, lambda
        );
        let sigma_sampler = SigmaSampler::new(nu, lambda);

        let init_splits: Vec<Split> = (0..p)
            .map(|i| Split::new_continuous(i as u16, i16::MAX - 1))
            .collect();
        // (x, y, init_splits)
        let block_size = f64::ceil((n as f64) / (num_cpus::get_physical() as f64)) as usize;
        let data_para = DataAndPara {
            x,
            y,
            w: None,
            n,
            p,
            init_splits,
            sigma_mu: 1.0,
            mu_mu: 0.0,
            alpha: 0.95,
            beta: 2.0,
            // prob_grow: 0.25,
            // prob_prune: 0.25,
            // prob_change: 0.50,
            prob_grow: 0.50,
            prob_prune: 0.50,
            prob_change: 0.0,
            n_eta: 5.0,
            block_size,
        };
        (data_para, sigma_sampler, y_min, y_max)
    }
}
