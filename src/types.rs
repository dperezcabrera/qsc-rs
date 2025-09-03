use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Call {
    pub from: String,
    pub contract: String,
    pub method: String,
    pub args: serde_json::Value,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Auth {
    pub alg: String,
    pub pk: String,
    pub sig: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tx {
    pub call: Call,
    pub timestamp: u128,
    pub auth: Auth,
    pub nonce: u64,
    pub chain_id: String,
    pub tx_hash: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: u64,
    pub parent: String,
    pub hash: String,
    pub timestamp: u128,
    pub validator_pk: String,
    pub validator_sig: String,
    pub txs: Vec<Tx>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RpcCall {
    pub from: String,
    pub contract: String,
    pub method: String,
    pub args: serde_json::Value,
    pub alg: String,
    pub pk: String,
    pub sig: String,
    pub nonce: u64,
    pub chain_id: String,
}
