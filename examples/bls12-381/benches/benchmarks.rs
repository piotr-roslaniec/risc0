#![allow(clippy::redundant_closure)]
#![allow(clippy::unit_arg)]

use bls12_381::setup_g1_addition_prover;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const BENCH_CASES: [usize; 7] = [2, 4, 8, 16, 32, 64, 128];

pub fn bench_g1_addition(c: &mut Criterion) {
    let mut rng = ark_std::test_rng();
    let mut group = c.benchmark_group("g1-addition");
    group.sample_size(10);

    for n_g1 in BENCH_CASES.iter() {
        let (mut prover, _expected_aggregate) = setup_g1_addition_prover(&mut rng, *n_g1);

        group.bench_with_input(BenchmarkId::new("G1", n_g1), n_g1, |b, _| {
            b.iter(|| prover.run().expect("Code should be provable"))
        });
    }
}

criterion_group!(benches, bench_g1_addition);

criterion_main!(benches);
