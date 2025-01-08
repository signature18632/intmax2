use ark_bn254::{Bn254, Fq, Fr, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr};
use intmax2_zkp::{
    common::signature::{flatten::FlatG2, sign::tx_tree_root_to_message_point},
    ethereum_types::{bytes32::Bytes32, u256::U256, u32limb_trait::U32LimbTrait},
};
use num_traits::identities::Zero;
use plonky2_bn254::fields::recover::RecoverFromX;
use plonky2_keccak::utils::solidity_keccak256;

/// Convert the message into a format that can be signed, using the same method as when signing the tx tree root.
fn message_to_point(mut message: Vec<u8>) -> G2Affine {
    let mut message_u32_slice = vec![];
    while message.len() % 4 != 0 {
        message.push(0);
    }

    for i in 0..message.len() / 4 {
        let mut u32_bytes = [0u8; 4];
        u32_bytes.copy_from_slice(&message[i * 4..(i + 1) * 4]);
        message_u32_slice.push(u32::from_be_bytes(u32_bytes));
    }
    let message_hash = solidity_keccak256(&message_u32_slice);
    let message = Bytes32::from_u32_slice(&message_hash);

    tx_tree_root_to_message_point(message)
}

pub fn sign_message(priv_key: Fr, message: Vec<u8>) -> anyhow::Result<FlatG2> {
    let message_point = message_to_point(message);

    let signature: G2Affine = (message_point * priv_key).into();

    Ok(FlatG2::from(signature))
}

fn check_pairing(g1s: Vec<G1Affine>, g2s: Vec<G2Affine>) -> bool {
    Bn254::multi_pairing(g1s, g2s).is_zero()
}

pub fn verify_signature(signature: FlatG2, pubkey: U256, message: Vec<u8>) -> anyhow::Result<()> {
    let pubkey_x: Fq = pubkey.clone().into();
    let pubkey_g1 = G1Affine::recover_from_x(pubkey_x);
    let g1_generator_inv = -G1Affine::generator();
    let message_g2 = message_to_point(message);
    let signature_g2 = G2Affine::from(signature);
    let g1s = vec![g1_generator_inv, pubkey_g1];
    let g2s = vec![signature_g2, message_g2];

    if !check_pairing(g1s, g2s) {
        anyhow::bail!("Invalid signature");
    }

    Ok(())
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

pub fn hex_to_bytes(hex: &str) -> anyhow::Result<Vec<u8>> {
    let data_hex = hex.strip_prefix("0x").ok_or(anyhow::anyhow!(
        "Data should be a hex string with 0x prefix"
    ))?;
    hex::decode(&data_hex).map_err(|err| anyhow::anyhow!("Failed to decode hex: {}", err))
}

#[cfg(test)]
mod test {
    use super::{sign_message, verify_signature};
    use intmax2_zkp::common::signature::key_set::KeySet;

    #[test]
    fn test_sign_verify() {
        let mut rnd = rand::thread_rng();
        let key = KeySet::rand(&mut rnd);
        let message = vec![1, 2, 3, 4, 5, 6, 7];
        let signature = sign_message(key.privkey, message.clone()).unwrap();
        assert!(verify_signature(signature.clone(), key.pubkey, message).is_ok());

        let different_message = vec![1, 2, 3, 4, 5, 6, 8];
        assert!(!verify_signature(signature.clone(), key.pubkey, different_message).is_ok());

        let different_message = vec![1, 2, 3, 4];
        assert!(!verify_signature(signature, key.pubkey, different_message).is_ok());
    }
}
