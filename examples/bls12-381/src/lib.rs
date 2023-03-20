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

use ark_bls12_381::{g1::Parameters, Bls12_381, Fq12Parameters, G1Projective, G2Projective};
use ark_ec::short_weierstrass_jacobian::GroupAffine;
use ark_ec::{PairingEngine, ProjectiveCurve};
use ark_ff::{Fp12, Zero};
use ark_std::rand::prelude::StdRng;
use ark_std::UniformRand;
use bls12_381_methods::{
    BLS12_381_ADDITION_ELF, BLS12_381_ADDITION_ID, BLS12_381_PAIRING_ELF, BLS12_381_PAIRING_ID,
};
use risc0_zkvm::{serde::to_vec, Prover};
use shared::{fp_to_bytes, g1_affine_to_bytes, g2_affine_to_bytes};

pub fn setup_g1_addition_prover<'a>(
    mut rng: &mut StdRng,
    n_g1: usize,
) -> (Prover<'a>, GroupAffine<Parameters>) {
    // Make the prover.
    let mut prover = Prover::new(BLS12_381_ADDITION_ELF, BLS12_381_ADDITION_ID)
        .expect("Prover should be constructed from matching method code & ID");

    // Add number of inputs to prover
    prover.add_input_u32_slice(&to_vec(&n_g1).expect("should be serializable"));

    // Add n g1 inputs to prover
    let mut expected_output = G1Projective::zero();
    for _ in 0..n_g1 {
        let g1 = G1Projective::rand(&mut rng).into_affine();
        let g1_bytes = g1_affine_to_bytes(&g1).to_vec();
        prover.add_input_u32_slice(&to_vec(&g1_bytes.len()).expect("should be serializable"));
        prover.add_input_u8_slice(&g1_bytes);

        expected_output += G1Projective::from(g1);
    }

    // Add aggregate to prover
    let expected_aggregate = expected_output.into_affine();
    let expected_aggregate_bytes = g1_affine_to_bytes(&expected_aggregate).to_vec();
    prover.add_input_u32_slice(
        &to_vec(&expected_aggregate_bytes.len()).expect("should be serializable"),
    );
    prover.add_input_u8_slice(&expected_aggregate_bytes);
    (prover, expected_aggregate)
}

pub fn setup_pairing_prover<'a>(mut rng: &mut StdRng) -> (Prover<'a>, Fp12<Fq12Parameters>) {
    let p = G1Projective::rand(&mut rng).into_affine();
    let q = G2Projective::rand(&mut rng).into_affine();

    // Make the prover.
    let mut prover = Prover::new(BLS12_381_PAIRING_ELF, BLS12_381_PAIRING_ID)
        .expect("Prover should be constructed from matching method code & ID");

    // Add inputs to prover
    let p_bytes = g1_affine_to_bytes(&p).to_vec();
    prover.add_input_u32_slice(&to_vec(&p_bytes.len()).expect("should be serializable"));
    prover.add_input_u8_slice(&p_bytes);

    let q_bytes = g2_affine_to_bytes(&q).to_vec();
    prover.add_input_u32_slice(&to_vec(&q_bytes.len()).expect("should be serializable"));
    prover.add_input_u8_slice(&q_bytes);

    let expected_pairing = Bls12_381::pairing(p, q);
    let expected_pairing_bytes = fp_to_bytes(&expected_pairing).to_vec();
    prover.add_input_u32_slice(
        &to_vec(&expected_pairing_bytes.len()).expect("should be serializable"),
    );
    prover.add_input_u8_slice(&expected_pairing_bytes);

    (prover, expected_pairing)
}

#[cfg(test)]
mod tests {
    use risc0_zkvm::serde::from_slice;

    use super::*;

    #[test]
    fn bls12_381_g1_addition() {
        let mut rng = ark_std::test_rng();
        let n_g1 = 1;

        let (mut prover, _expected_aggregate) = setup_g1_addition_prover(&mut rng, n_g1);

        let receipt = prover.run().expect("Code should be provable");
        receipt
            .verify(&BLS12_381_ADDITION_ID)
            .expect("Proven code should verify");

        let aggregate_matches: bool =
            from_slice(&receipt.journal).expect("Journal should contain an boolean element");

        assert!(aggregate_matches);
    }

    #[test]
    fn bls12_381_pairing() {
        let mut rng = ark_std::test_rng();

        let (mut prover, _expected_output) = setup_pairing_prover(&mut rng);

        let receipt = prover.run().expect("Code should be provable");
        receipt
            .verify(&BLS12_381_PAIRING_ID)
            .expect("Proven code should verify");

        let pairing_matches: bool =
            from_slice(&receipt.journal).expect("Journal should contain an boolean element");

        assert!(pairing_matches);
    }
}
