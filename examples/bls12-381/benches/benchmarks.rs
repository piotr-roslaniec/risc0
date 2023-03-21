#![allow(clippy::redundant_closure)]
#![allow(clippy::unit_arg)]

use bls12_381::setup_g1_addition_prover;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const BENCH_CASES: [usize; 6] = [2, 4, 8, 16, 32, 64];

pub fn bench_bls12_381(c: &mut Criterion) {
    let mut rng = ark_std::test_rng();
    let mut group = c.benchmark_group("bls12-381");
    group.sample_size(10);

    for n_g1 in BENCH_CASES.iter() {
        group.bench_function(BenchmarkId::new("g1-addition", n_g1), |b| {
            let (mut prover, _expected_aggregate) = setup_g1_addition_prover(&mut rng, *n_g1);
            b.iter(|| prover.run().expect("Code should be provable"))
        });
    }
}

criterion_group!(benches, bench_bls12_381);

criterion_main!(benches);
