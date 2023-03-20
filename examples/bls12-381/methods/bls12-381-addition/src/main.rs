// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use ark_bls12_381::G1Projective;
use ark_ff::Zero;
use risc0_zkvm::guest::env;

use shared::{g1_affine_from_bytes};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let n_g1: usize = env::read();

    let mut aggregate = G1Projective::zero();
    for _ in 0..n_g1 {
        let g1_bytes_len: usize = env::read();
        let g1_bytes: &[u8] = env::read_slice(g1_bytes_len);
        let g1 = g1_affine_from_bytes(g1_bytes);
        let g1 = G1Projective::from(g1);
        aggregate += g1;
    }

    let expected_agg_bytes_len: usize = env::read();
    let expected_agg_bytes: &[u8] = env::read_slice(expected_agg_bytes_len);
    let expected_agg = g1_affine_from_bytes(expected_agg_bytes);
    let expected_agg = G1Projective::from(expected_agg);

    let aggregate_matches = aggregate == expected_agg;
    assert!(aggregate_matches);

    env::commit(&aggregate_matches);
}
