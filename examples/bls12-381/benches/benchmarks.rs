#![allow(clippy::redundant_closure)]
#![allow(clippy::unit_arg)]

use bls12_381::{setup_g1_addition_prover, setup_pairing_prover};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const BENCH_CASES: [usize; 6] = [2, 4, 8, 16, 32, 64];

pub fn bench_bls12_381_g1_addition(c: &mut Criterion) {
    let mut rng = ark_std::test_rng();
    let mut group = c.benchmark_group("bls12-381-addition");
    group.sample_size(10);

    for n_g1 in BENCH_CASES.iter() {
        group.bench_function(BenchmarkId::new("g1-addition", n_g1), |b| {
            let (mut prover, _) = setup_g1_addition_prover(&mut rng, *n_g1);
            b.iter(|| prover.run().expect("Code should be provable"))
        });

        // If I create the prover here, and pass it into the bench_function closure, it throws an error:
        // "Verification failed: Journal and seal root mismatch detected
        // Uncomment to reproduce:
        // let (mut prover, _) = setup_g1_addition_prover(&mut rng, *n_g1);
        // group.bench_function(BenchmarkId::new("g1-addition", n_g1), |b| {
        //     b.iter(|| prover.run().expect("Code should be provable"))
        // });
    }
}

// This benchmark fails (see comments in tests in src/lib.rs)
pub fn bench_bls12_381_pairing(c: &mut Criterion) {
    let mut rng = ark_std::test_rng();
    let mut group = c.benchmark_group("bls12-381-pairing");
    group.sample_size(10);

    group.bench_function("pairing", |b| {
        let (mut prover, _) = setup_pairing_prover(&mut rng);
        b.iter(|| prover.run().expect("Code should be provable"))
    });
}

criterion_group!(
    benches,
    bench_bls12_381_g1_addition,
    bench_bls12_381_pairing
);

criterion_main!(benches);
