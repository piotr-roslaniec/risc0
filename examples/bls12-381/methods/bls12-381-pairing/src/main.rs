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

use ark_bls12_381::{Bls12_381};
use ark_ec::{PairingEngine};
use risc0_zkvm::guest::env;

use shared::{g1_affine_from_bytes, g2_affine_from_bytes, fp_from_bytes};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let p_bytes_len: usize = env::read();
    let p_bytes: &[u8] = env::read_slice(p_bytes_len);
    let p = g1_affine_from_bytes(p_bytes);

    let q_bytes_len: usize = env::read();
    let q_bytes: &[u8] = env::read_slice(q_bytes_len);
    let q = g2_affine_from_bytes(q_bytes);

    let expected_pairing_bytes_len: usize = env::read();
    let expected_pairing_bytes: &[u8] = env::read_slice(expected_pairing_bytes_len);
    let expected_pairing = fp_from_bytes(expected_pairing_bytes);

    let pairing = Bls12_381::pairing(p, q);

    let pairing_matches = expected_pairing == pairing;
    assert!(pairing_matches);

    env::commit(&pairing_matches);
}
