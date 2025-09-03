use sha3::{Digest as ShaDigest, Sha3_512};
use blake2::digest::consts::U64;
use blake2::Blake2b;
use once_cell::sync::Lazy;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy)]
pub enum HashAlg { Sha3_512, Blake2b512 }

static HASH_ALG: Lazy<HashAlg> = Lazy::new(|| {
    match std::env::var("QSC_HASH_ALG").unwrap_or_else(|_| "sha3-512".into()).to_lowercase().as_str() {
        "blake2b-512" | "blake2b512" | "blake2" => HashAlg::Blake2b512,
        _ => HashAlg::Sha3_512,
    }
});

pub fn current_hash_alg() -> HashAlg { *HASH_ALG }

pub fn now_ms() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn hash_hex(data: &[u8]) -> String {
    match *HASH_ALG {
        HashAlg::Sha3_512 => {
            let mut h = Sha3_512::new();
            ShaDigest::update(&mut h, data);
            hex::encode(h.finalize())
        }
        HashAlg::Blake2b512 => {
            let mut h = Blake2b::<U64>::new();
            // Disambiguate the trait call explicitly
            use blake2::digest::Update as BlakeUpdate;
            BlakeUpdate::update(&mut h, data);
            hex::encode(h.finalize())
        }
    }
}
