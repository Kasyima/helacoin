use ecdsa::{signature::Signer, Signature as ECDSASignature, SigningKey, VerifyingKey};
use k256::Secp256k1;
use rand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature(ECDSASignature<Secp256k1>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicKey(VerifyingKey<Secp256k1>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateKey(#[serde(with = "signkey_serde")] SigningKey<Secp256k1>);
mod signkey_serde {
    use serde::{Deserialize, Serializer};
    pub fn serialize<S>(
        key: &super::SigningKey<super::Secp256k1>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // convert the key into a slice of bytes, then serialize bytes -- which serde already knows how to do.
        serializer.serialize_bytes(&key.to_bytes())
    }
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<super::SigningKey<super::Secp256k1>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // the line tells the deserializer to deserialize this bit of data as a vector,
        let bytes: Vec<u8> = Vec::deserialize(deserializer)?;
        // then the parsing is done with the from_slice() found on SigningKey
        Ok(super::SigningKey::from_slice(&bytes).unwrap())
    }
}

impl PrivateKey {
    // pub fn new_key() -> Self {
    //     PrivateKey(SigningKey::random(&mut rand::rng()))
    // }
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.verifying_key().clone())
    }
}
