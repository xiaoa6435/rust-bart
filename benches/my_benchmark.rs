use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use rust_bart::{
    data_and_para::DataAndPara,
    sampler_par::{
        change_proposer::ChangeProposer, grow_proposer::GrowProposer,
        prune_proposer::PruneProposer, tree_mutation_proposer::TreeMutationProposer,
    },
    tree::tree::Tree, sampler::sigma_sampler::SigmaSampler,
};

fn propoal_tree(
    mut rng: &mut rand::rngs::ThreadRng,
    data_para: &DataAndPara,
    forest: &mut Vec<Tree>,
    resid: &mut Vec<f64>,
    sigma_sampler: &mut SigmaSampler,
    tau: f64,
) {
    (0..100).for_each(|_i| {
        let sigma = sigma_sampler.sample(&resid, &mut rng);
        forest.iter_mut().for_each(|tree| {
            let p = rng.gen_range(0.0..1.0);
            if p < data_para.prob_grow {
                let mut grow = GrowProposer {
                    tree,
                    resid,
                    data_para: &data_para,
                    sigma,
                    tau,
                };
                grow.sample(&mut rng);
            } else if p < data_para.prob_grow + data_para.prob_prune {
                let mut prune = PruneProposer {
                    tree,
                    resid,
                    data_para: &data_para,
                    sigma,
                    tau,
                };
                prune.sample(&mut rng);
            } else {
                let mut change = ChangeProposer {
                    tree,
                    resid,
                    data_para: &data_para,
                    sigma,
                    tau,
                };
                change.sample(&mut rng);
            }
        })
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    // rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let (n, p) = (100000, 20);
    let (data_para, mut sigma_sampler, y_min, y_max) = DataAndPara::test_data(n, p);
    let mut resid = data_para.y.clone();
    let ntree = 200;
    let mut forest: Vec<Tree> = (0..ntree).map(|_| Tree::new(n)).collect();
    // let mut tree = forest[0].clone();
    let mut rng = thread_rng();
    let tau = (y_max - y_min) / (2.0 * 2.0 * f64::sqrt(ntree as f64));
    // let mut sigma = sigma_sampler.sample(&resid, &mut rng);

    c.bench_function("propoal_tree", |b| {
        b.iter(|| propoal_tree(&mut rng, &data_para, &mut forest, &mut resid, &mut sigma_sampler, tau))
    });
    // .measurement_time(Duration::from_secs(60));
    // .measurement_time(Duration::from_millis(1000));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// time:   [3.1723 ms 3.2218 ms 3.2714 ms]
// time:   [4.6191 ms 4.6646 ms 4.7107 ms]
