pub mod change_proposer;
pub mod grow_proposer;
pub mod prune_proposer;
pub mod sigma_sampler;
pub mod tree_mutation_proposer;

#[cfg(test)]
mod tests {

    use std::time::SystemTime;

    use env_logger::Env;
    use log::{debug, info, warn};
    use rand::{thread_rng, Rng};

    use crate::{data_and_para::DataAndPara, tree::tree::Tree};

    use super::{
        change_proposer::ChangeProposer, grow_proposer::GrowProposer,
        prune_proposer::PruneProposer, tree_mutation_proposer::TreeMutationProposer,
    };

    fn check_resid_invariant(
        forest: &Vec<Tree>,
        resid: &Vec<f64>,
        data_para: &DataAndPara,
    ) -> bool {
        resid.iter().enumerate().all(|(idx, &r)| {
            let pred: f64 = forest
                .iter()
                .map(|tree| tree.predict_mu(&data_para.x, idx))
                .sum();
            data_para.y[idx] - pred - r < 1e-6
        })
    }

    fn check_tree_leaf_idx_invariant(tree: &Tree, x: &Vec<Vec<i16>>) -> bool {
        tree.leaf_idx
            .iter()
            .enumerate()
            .all(|(idx, &leaf_idx)| tree.predict(x, idx) == leaf_idx)
    }

    // 0.406s/iter
    #[test]
    fn test_idx() {
        env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

        let (n, p) = (200000, 20);
        let (data_para, mut sigma_sampler, y_min, y_max) = DataAndPara::test_data(n, p);
        let mut resid = data_para.y.clone();
        let ntree = 200;
        let mut forest: Vec<Tree> = (0..ntree).map(|_| Tree::new(n)).collect();
        let mut rng = thread_rng();
        let tau = (y_max - y_min) / (2.0 * 2.0 * f64::sqrt(ntree as f64));
        let now = SystemTime::now();
        (0..1000).for_each(|i| {
            let sigma = sigma_sampler.sample(&resid, &mut rng);
            // warn!(
            //     "{:4}-rmse:{:.3}, sigma:{:.3}, tau:{:?}, resid is valid:{}",
            //     i, sigma_sampler.rmse, sigma, tau,
            //     check_resid_invariant(&forest, &resid, &data_para)
            // );
            warn!(
                "{:4}-rmse:{:.3}, sigma:{:.3}, {:.3}s/iter",
                i, sigma_sampler.rmse, sigma, now.elapsed().unwrap().as_secs_f64() / (i as f64),
            );
            debug!(
                "resid is valid:{}",
                check_resid_invariant(&forest, &resid, &data_para)
            );
            forest.iter_mut().for_each(|tree| {
                debug!(
                    "tree leaf idx is valid-bef:{}",
                    check_tree_leaf_idx_invariant(tree, &data_para.x)
                );
                info!("old tree:{}", tree);
                let p = rng.gen_range(0.0..1.0);
                if p < data_para.prob_grow {
                    let mut grow = GrowProposer {
                        tree,
                        resid: &mut resid,
                        data_para: &data_para,
                        sigma,
                        tau,
                    };
                    grow.sample(&mut rng);
                } else if p < data_para.prob_grow + data_para.prob_prune {
                    let mut prune = PruneProposer {
                        tree,
                        resid: &mut resid,
                        data_para: &data_para,
                        sigma,
                        tau,
                    };
                    prune.sample(&mut rng);
                } else {
                    let mut change = ChangeProposer {
                        tree,
                        resid: &mut resid,
                        data_para: &data_para,
                        sigma,
                        tau,
                    };
                    change.sample(&mut rng);
                }
                info!("new tree:{}", tree);
                debug!(
                    "tree leaf idx is valid-aft:{}",
                    check_tree_leaf_idx_invariant(tree, &data_para.x)
                );
            })
        })
    }

    
}
