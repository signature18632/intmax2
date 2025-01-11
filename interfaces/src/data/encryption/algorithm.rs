use aes::{
    cipher::{KeyIvInit, StreamCipher},
    Aes128,
};
use alloy_primitives::B128;
use ctr::Ctr64BE;
use intmax2_zkp::{
    common::signature::key_set::KeySet,
    ethereum_types::{u256::U256, u32limb_trait::U32LimbTrait},
};
use rand::Rng;
use reth_ecies::ECIESError;

use super::{
    message::EncryptedMessage,
    utils::{ecdh_x, hmac_sha256, kdf, sha256, U256_SIZE},
};

pub use alloy_primitives::bytes::BytesMut;

pub(crate) fn encrypt(_pubkey: U256, data: &[u8]) -> Vec<u8> {
    // todo: add encryption
    data.to_vec()
}

pub(crate) fn decrypt(_key: KeySet, encrypted_data: &[u8]) -> anyhow::Result<Vec<u8>> {
    // todo: add decryption
    Ok(encrypted_data.to_vec())
}

pub struct Ecies {
    pub key: KeySet,
    pub remote_public_key: Option<U256>,
}

impl Ecies {
    pub fn new(key: KeySet, remote_public_key: U256) -> Self {
        Self {
            key,
            remote_public_key: Some(remote_public_key),
        }
    }

    pub fn new_server(key: KeySet) -> Self {
        Self {
            key,
            remote_public_key: None,
        }
    }

    pub fn encrypt_message(&self, data: &[u8], out: &mut BytesMut) {
        let mut rng = rand::thread_rng();

        out.reserve(U256_SIZE + 16 + data.len() + 32);

        let total_size: u16 = u16::try_from(U256_SIZE + 16 + data.len() + 32).unwrap();
        out.extend_from_slice(&total_size.to_be_bytes());

        let key = KeySet::rand(&mut rng);
        out.extend_from_slice(&key.pubkey.to_bytes_be()); // 32 bytes

        let remote_public_key = self.remote_public_key.unwrap();
        let x = ecdh_x(&remote_public_key, &key.privkey);
        let mut key = [0u8; 32];
        kdf(x, &[], &mut key);

        let enc_key = B128::from_slice(&key[..16]);
        let mac_key = sha256(&key[16..32]);

        let iv: B128 = rng.gen();
        let mut encryptor = Ctr64BE::<Aes128>::new((&enc_key.0).into(), (&iv.0).into());

        let mut encrypted = data.to_vec();
        encryptor.apply_keystream(&mut encrypted);

        let tag = hmac_sha256(
            mac_key.as_ref(),
            &[iv.as_slice(), &encrypted],
            &total_size.to_be_bytes(),
        );

        out.extend_from_slice(iv.as_slice());
        out.extend_from_slice(&encrypted);
        out.extend_from_slice(tag.as_ref());
    }

    pub fn decrypt_message<'a>(&self, data: &'a mut [u8]) -> Result<&'a mut [u8], ECIESError> {
        // parse the encrypted message from bytes
        let encrypted_message = EncryptedMessage::parse(data)?;

        // derive keys from the secret key and the encrypted message
        let keys = encrypted_message.derive_keys(&self.key.privkey);

        // check message integrity and decrypt the message
        encrypted_message.check_and_decrypt(keys)
    }
}

#[cfg(test)]
mod test {
    use intmax2_zkp::common::signature::key_set::KeySet;

    use super::{BytesMut, Ecies};

    #[test]
    fn test_ecies_encryption() {
        let mut rand = rand::thread_rng();
        let client_key = KeySet::rand(&mut rand);
        let server_key = KeySet::rand(&mut rand);
        let client = Ecies::new(client_key, server_key.pubkey);

        let data = b"hello world";
        println!("data: {:?}", data.to_vec());

        let mut encrypted_data = BytesMut::new();
        client.encrypt_message(data, &mut encrypted_data);
        println!("encrypted data: {:#02x}", encrypted_data);

        // let version = 1u8;
        // let version_data = version.to_be_bytes();

        let server = Ecies::new_server(server_key);
        let decrypted_data = server.decrypt_message(&mut encrypted_data).unwrap();
        println!("decrypted data: {:?}", decrypted_data);

        assert_eq!(data.to_vec(), decrypted_data);
    }
}
