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

use ark_bls12_381::{G1Affine, G1Projective};
use ark_ec::ProjectiveCurve;
use ark_ff::PrimeField;
use ark_ff::ToBytes;
use ark_std::UniformRand;
use bls12_381_methods::{BLS12_381_ADDITION_ELF, BLS12_381_ADDITION_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover,
};

fn affine_to_bytes_calldata(g1: G1Affine) -> [u8; 96] {
    let x = g1.x;
    let y = g1.y;

    let mut x_bytes = [0u8; 48];
    let mut y_bytes = [0u8; 48];

    x.into_repr().write(&mut x_bytes[..]).unwrap();
    y.into_repr().write(&mut y_bytes[..]).unwrap();

    let bytes: [u8; 96] = [x_bytes, y_bytes].concat().try_into().unwrap();
    bytes
}

fn main() {
    let mut rng = ark_std::test_rng();

    let a = G1Projective::rand(&mut rng);
    let b = G1Projective::rand(&mut rng);

    let c: G1Projective = a + b;

    let a = a.into_affine();
    let b = b.into_affine();
    let c = G1Affine::from(c);

    let a_bytes = affine_to_bytes_calldata(a).to_vec();
    let b_bytes = affine_to_bytes_calldata(b).to_vec();
    let c_bytes = affine_to_bytes_calldata(c).to_vec();

    let a_bytes_len = a_bytes.len();
    let b_bytes_len = b_bytes.len();
    let c_bytes_len = c_bytes.len();

    // Make the prover.
    let mut prover = Prover::new(BLS12_381_ADDITION_ELF, BLS12_381_ADDITION_ID)
        .expect("Prover should be constructed from matching method code & ID");

    prover.add_input_u32_slice(&to_vec(&a_bytes_len).expect("should be serializable"));
    prover.add_input_u8_slice(&a_bytes);
    prover.add_input_u32_slice(&to_vec(&b_bytes_len).expect("should be serializable"));
    prover.add_input_u8_slice(&b_bytes);
    prover.add_input_u32_slice(&to_vec(&c_bytes_len).expect("should be serializable"));
    prover.add_input_u8_slice(&c_bytes);

    // Run prover & generate receipt
    let receipt = prover.run().expect("Code should be provable");

    receipt
        .verify(&BLS12_381_ADDITION_ID)
        .expect("Proven code should verify");

    let journal = &receipt.journal;
    let output: bool = from_slice(&journal).expect("Journal should contain an Outputs object");

    println!("\nResult: {}\n", output);
}

#[cfg(test)]
mod tests {
    use ark_bls12_381::{G1Affine, G1Projective};
    use ark_ec::ProjectiveCurve;
    use ark_ff::PrimeField;
    use ark_ff::{bytes::FromBytes, ToBytes};
    use ark_std::UniformRand;
    use bls12_381_methods::{BLS12_381_ADDITION_ELF, BLS12_381_ADDITION_ID};
    use risc0_zkvm::{
        serde::{from_slice, to_vec},
        Prover,
    };

    use crate::affine_to_bytes_calldata;

    #[test]
    fn main() {
        let mut rng = ark_std::test_rng();

        let a = G1Projective::rand(&mut rng);
        let b = G1Projective::rand(&mut rng);

        let c: G1Projective = a + b;

        let a = a.into_affine();
        let b = b.into_affine();
        let c = G1Affine::from(c);

        let a_bytes = affine_to_bytes_calldata(a).to_vec();
        let b_bytes = affine_to_bytes_calldata(b).to_vec();
        let c_bytes = affine_to_bytes_calldata(c).to_vec();

        let a_bytes_len = a_bytes.len();
        let b_bytes_len = b_bytes.len();
        let c_bytes_len = c_bytes.len();

        // Make the prover.
        let mut prover = Prover::new(BLS12_381_ADDITION_ELF, BLS12_381_ADDITION_ID)
            .expect("Prover should be constructed from matching method code & ID");

        prover.add_input_u32_slice(&to_vec(&a_bytes_len).expect("should be serializable"));
        prover.add_input_u8_slice(&a_bytes);
        prover.add_input_u32_slice(&to_vec(&b_bytes_len).expect("should be serializable"));
        prover.add_input_u8_slice(&b_bytes);
        prover.add_input_u32_slice(&to_vec(&c_bytes_len).expect("should be serializable"));
        prover.add_input_u8_slice(&c_bytes);

        // Run prover & generate receipt
        let receipt = prover.run().expect("Code should be provable");

        receipt
            .verify(&BLS12_381_ADDITION_ID)
            .expect("Proven code should verify");

        let journal = &receipt.journal;
        let output: bool = from_slice(&journal).expect("Journal should contain an Outputs object");

        assert!(output);
    }
}
