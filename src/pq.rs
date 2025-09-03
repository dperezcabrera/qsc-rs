use pqcrypto_dilithium::dilithium3::{keypair, SecretKey, PublicKey, DetachedSignature, detached_sign, verify_detached_signature};
use pqcrypto_traits::sign::{SecretKey as SecretKeyTrait, PublicKey as PublicKeyTrait, DetachedSignature as DetachedSigTrait};
use crate::util::hash_hex;

pub fn keygen_mldsa3() -> (Vec<u8>, Vec<u8>) {
    let (pk, sk) = keypair();
    (sk.as_bytes().to_vec(), pk.as_bytes().to_vec())
}
pub fn keypair_mldsa3() -> (Vec<u8>, Vec<u8>) { keygen_mldsa3() }

pub fn sign_mldsa3(msg: &[u8], sk: &[u8]) -> Vec<u8> {
    let sk = SecretKey::from_bytes(sk).expect("bad sk bytes");
    let sig = detached_sign(msg, &sk);
    sig.as_bytes().to_vec()
}
pub fn verify_mldsa3(msg: &[u8], sig: &[u8], pk: &[u8]) -> bool {
    let pk = match PublicKey::from_bytes(pk) { Ok(pk) => pk, Err(_) => return false };
    let sig = match DetachedSignature::from_bytes(sig) { Ok(s) => s, Err(_) => return false };
    verify_detached_signature(&sig, msg, &pk).is_ok()
}

pub fn address_from_pk(pk: &[u8]) -> String {
    hash_hex(pk)
}
