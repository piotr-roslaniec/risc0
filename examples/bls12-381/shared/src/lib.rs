use ark_bls12_381::{Fq12, G1Affine, G2Affine};
use ark_ff::{FromBytes, PrimeField, ToBytes};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

pub type G1Bytes = [u8; 48];
pub type G2Bytes = [u8; 96];
pub type Fq12Bytes = [u8; 576];

pub fn g1_affine_to_bytes(g1: G1Affine) -> [u8; 96] {
    let x = g1.x;
    let y = g1.y;

    let mut x_bytes = [0u8; 48];
    let mut y_bytes = [0u8; 48];

    x.into_repr().write(&mut x_bytes[..]).unwrap();
    y.into_repr().write(&mut y_bytes[..]).unwrap();

    let bytes: [u8; 96] = [x_bytes, y_bytes].concat().try_into().unwrap();
    bytes
}

pub fn g1_affine_from_bytes(bytes: &[u8]) -> G1Affine {
    let x_bytes: [u8; 48] = bytes[0..48].try_into().unwrap();
    let y_bytes: [u8; 48] = bytes[48..96].try_into().unwrap();

    let x = ark_bls12_381::Fq::read(&x_bytes[..]).unwrap();
    let y = ark_bls12_381::Fq::read(&y_bytes[..]).unwrap();

    G1Affine::new(x, y, false)
}

pub fn g2_affine_to_bytes(g2: &G2Affine) -> G2Bytes {
    let mut bytes = Vec::new();
    g2.serialize(&mut bytes).unwrap();
    let bytes: G2Bytes = bytes.try_into().unwrap();
    bytes
}

pub fn g2_affine_from_bytes(bytes: &[u8]) -> G2Affine {
    let bytes: G2Bytes = bytes.try_into().unwrap();
    G2Affine::deserialize(&bytes[..]).unwrap()
}

pub fn fp_to_bytes(fp: &Fq12) -> Fq12Bytes {
    let mut bytes = Vec::new();
    fp.serialize(&mut bytes).unwrap();
    let bytes: Fq12Bytes = bytes.try_into().unwrap();
    bytes
}

pub fn fp_from_bytes(bytes: &[u8]) -> Fq12 {
    let bytes: Fq12Bytes = bytes.try_into().unwrap();
    Fq12::deserialize(&bytes[..]).unwrap()
}

#[cfg(test)]
mod tests {
    use ark_bls12_381::{G1Projective, G2Projective};
    use ark_ec::ProjectiveCurve;
    use ark_ff::Fp12;
    use ark_std::UniformRand;

    use super::*;

    #[test]
    fn g1_affine_serializes() {
        let mut rng = ark_std::test_rng();

        let g1 = G1Projective::rand(&mut rng).into_affine();
        let g1_bytes = g1_affine_to_bytes(g1);
        assert_eq!(g1_bytes.len(), 48);

        let g1_new = g1_affine_from_bytes(&g1_bytes);
        assert_eq!(g1, g1_new);
    }

    #[test]
    fn g2_affine_serializes() {
        let mut rng = ark_std::test_rng();

        let g2 = G2Projective::rand(&mut rng).into_affine();
        let g2_bytes = g2_affine_to_bytes(&g2);
        assert_eq!(g2_bytes.len(), 96);

        let g2_new = g2_affine_from_bytes(&g2_bytes);
        assert_eq!(g2, g2_new);
    }

    #[test]
    fn fp_serializes() {
        let mut rng = ark_std::test_rng();

        let fp = Fp12::rand(&mut rng);
        let fp_bytes = fp_to_bytes(&fp);
        assert_eq!(fp_bytes.len(), 576);

        let fp_new = fp_from_bytes(&fp_bytes);
        assert_eq!(fp, fp_new);
    }
}
