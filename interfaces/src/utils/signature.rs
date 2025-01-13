use ark_bn254::{Bn254, Fq, Fr, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr};
use intmax2_zkp::{
    common::signature::{flatten::FlatG2, key_set::KeySet},
    ethereum_types::{bytes32::Bytes32, u256::U256, u32limb_trait::U32LimbTrait},
};
use num_traits::identities::Zero;
use plonky2::field::{goldilocks_field::GoldilocksField, types::Field};
use plonky2_bn254::{
    curves::g2::G2Target, fields::recover::RecoverFromX, utils::hash_to_g2::HashToG2,
};
use serde::{Deserialize, Serialize};
use sha2::Digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub pubkey: U256,
    pub expiry: u64,
    pub signature: FlatG2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignContent {
    pub pubkey: U256,
    pub content: Vec<u8>,
    pub expiry: u64,
}

impl Auth {
    pub fn sign(key: KeySet, time_to_expiry: u64, content: &[u8]) -> Self {
        let expiry = current_time() + time_to_expiry;
        let sign_content = SignContent {
            pubkey: key.pubkey,
            content: content.to_vec(),
            expiry,
        };
        let serialized = bincode::serialize(&sign_content).unwrap();
        let digest = sha2::Sha256::digest(&serialized);
        let hash = Bytes32::from_bytes_be(&digest);
        let signature = sign_message(key.privkey, hash).unwrap();
        Auth {
            pubkey: key.pubkey,
            expiry,
            signature,
        }
    }

    pub fn verify(&self, content: &[u8]) -> anyhow::Result<()> {
        if self.expiry < current_time() {
            anyhow::bail!("Signature expired");
        }
        let sign_content = SignContent {
            pubkey: self.pubkey,
            content: content.to_vec(),
            expiry: self.expiry,
        };
        let serialized = bincode::serialize(&sign_content).unwrap();
        let digest = sha2::Sha256::digest(&serialized);
        let hash = Bytes32::from_bytes_be(&digest);
        verify_signature(self.signature.clone(), self.pubkey, hash)
    }
}

pub fn sign_message(priv_key: Fr, hash: Bytes32) -> anyhow::Result<FlatG2> {
    let message_point = hash_to_message_point(hash);
    let signature: G2Affine = (message_point * priv_key).into();
    Ok(FlatG2::from(signature))
}

pub fn verify_signature(signature: FlatG2, pubkey: U256, hash: Bytes32) -> anyhow::Result<()> {
    let pubkey_x: Fq = pubkey.into();
    let pubkey_g1 = G1Affine::recover_from_x(pubkey_x);
    let g1_generator_inv = -G1Affine::generator();
    let message_g2 = hash_to_message_point(hash);
    let signature_g2 = G2Affine::from(signature);
    let g1s = vec![g1_generator_inv, pubkey_g1];
    let g2s = vec![signature_g2, message_g2];
    if !check_pairing(g1s, g2s) {
        anyhow::bail!("Invalid signature");
    }
    Ok(())
}

fn current_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn hash_to_message_point(hash: Bytes32) -> G2Affine {
    let elements = hash
        .to_u32_vec()
        .iter()
        .map(|x| GoldilocksField::from_canonical_u32(*x))
        .collect::<Vec<_>>();
    G2Target::<GoldilocksField, 2>::hash_to_g2(&elements)
}

fn check_pairing(g1s: Vec<G1Affine>, g2s: Vec<G2Affine>) -> bool {
    Bn254::multi_pairing(g1s, g2s).is_zero()
}

#[cfg(test)]
mod test {
    use super::{sign_message, verify_signature};
    use intmax2_zkp::{
        common::signature::key_set::KeySet,
        ethereum_types::{bytes32::Bytes32, u32limb_trait::U32LimbTrait as _},
    };

    #[test]
    fn test_sign_verify() {
        let mut rnd = rand::thread_rng();
        let key = KeySet::rand(&mut rnd);
        let hash = Bytes32::rand(&mut rnd);
        let signature = sign_message(key.privkey, hash).unwrap();
        assert!(verify_signature(signature.clone(), key.pubkey, hash).is_ok());
    }

    #[test]
    fn test_auth_verify() {
        let mut rnd = rand::thread_rng();
        let key = KeySet::rand(&mut rnd);
        let content = b"test";
        let auth = super::Auth::sign(key, 10, content);
        assert!(auth.verify(content).is_ok());
    }
}
