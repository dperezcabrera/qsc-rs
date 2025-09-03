use serde::{Serialize, Deserialize};
use crate::types::Block;
use crate::pq;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub id: String,
    pub url: String,
    pub pk: String,
}

#[derive(Debug, Clone)]
pub struct PoAConfig {
    pub validators: Vec<Validator>,
    pub slot_ms: u64,
}

impl PoAConfig {
    pub fn from_env() -> Option<Self> {
        let raw = std::env::var("QSC_VALIDATORS_JSON").ok()?;
        let validators: Vec<Validator> = serde_json::from_str(&raw).ok()?;
        let slot_ms = std::env::var("QSC_SLOT_MS").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);
        Some(Self { validators, slot_ms })
    }
    pub fn expected_leader(&self, height_next: u64) -> &Validator {
        let n = self.validators.len().max(1);
        &self.validators[(height_next as usize) % n]
    }
}

pub fn verify_block_poa(cfg: &PoAConfig, parent_hash: &str, block: &Block) -> Result<(), String> {
    let expected = cfg.expected_leader(block.height);
    if !block.validator_pk.eq_ignore_ascii_case(&expected.pk) {
        return Err(format!("unexpected leader: got {}, expected {}", block.validator_pk, expected.pk));
    }
    if block.parent != parent_hash {
        return Err(format!("bad parent: {} != {}", block.parent, parent_hash));
    }
    let sig = hex::decode(&block.validator_sig).map_err(|_| "bad leader sig hex".to_string())?;
    let pk  = hex::decode(&block.validator_pk).map_err(|_| "bad leader pk hex".to_string())?;
    if !pq::verify_mldsa3(block.hash.as_bytes(), &sig, &pk) {
        return Err("invalid leader signature".into());
    }
    Ok(())
}
