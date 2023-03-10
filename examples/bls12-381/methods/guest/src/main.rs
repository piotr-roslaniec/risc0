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

use ark_bls12_381::{G1Affine, G1Projective};
use ark_ff::bytes::FromBytes;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn bytes_calldata_to_affine(bytes: [u8; 96]) -> G1Affine {
    let x_bytes: [u8; 48] = bytes[0..48].try_into().unwrap();
    let y_bytes: [u8; 48] = bytes[48..96].try_into().unwrap();

    let x = ark_bls12_381::Fq::read(&x_bytes[..]).unwrap();
    let y = ark_bls12_381::Fq::read(&y_bytes[..]).unwrap();

    G1Affine::new(x, y, false)
}

pub fn main() {
    let a_bytes_len: usize = env::read();
    let a_bytes: &[u8] = env::read_slice(a_bytes_len);
    let b_bytes_len: usize = env::read();
    let b_bytes: &[u8] = env::read_slice(b_bytes_len);
    let c_bytes_len: usize = env::read();
    let c_bytes: &[u8] = env::read_slice(c_bytes_len);

    let a = bytes_calldata_to_affine(a_bytes.try_into().unwrap());
    let b = bytes_calldata_to_affine(b_bytes.try_into().unwrap());
    let c = bytes_calldata_to_affine(c_bytes.try_into().unwrap());

    let a = G1Projective::from(a);
    let b = G1Projective::from(b);
    let c = G1Projective::from(c);

    let my_c = a + b;
    let c_matches = c == my_c;

    env::commit(&c_matches);
}
