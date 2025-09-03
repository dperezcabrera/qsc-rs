use crate::contracts::{self, Ctx};
use crate::pq;
use crate::storage;
use crate::types::{Auth, Block, Call, Tx};
use crate::util::{hash_hex, now_ms};

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

pub type SharedRuntime = Arc<Runtime>;

pub struct Runtime {
    pub ctx: Mutex<Ctx>,
    pub mempool: Mutex<Vec<Tx>>,
    pub chain: Mutex<Vec<Block>>,
    pub nonces: Mutex<HashMap<String, u64>>,
    pub chain_id: String,
    pub validator_sk: Vec<u8>,
    pub validator_pk: Vec<u8>,
}

impl Runtime {
    pub fn produce_block(&self) -> Block {
        let parent = self.chain.lock().last().cloned().expect("genesis must exist");
        let height = parent.height + 1;
        let timestamp = now_ms();

        let txs = {
            let mut pool = self.mempool.lock();
            let max_block: usize = std::env::var("QSC_MAX_TX_PER_BLOCK").ok().and_then(|s| s.parse().ok()).unwrap_or(100);
            let take = std::cmp::min(max_block, pool.len());
            pool.drain(..take).collect::<Vec<_>>()
        };

        for tx in &txs {
            let _ = self.dispatch_mut(&tx.call);
            self.inc_nonce(&tx.call.from);
        }

        let header_str = format!("{}|{}|{}|{}", parent.hash, height, txs.len(), timestamp);
        let hash = hash_hex(header_str.as_bytes());

        let sig = pq::sign_mldsa3(hash.as_bytes(), &self.validator_sk);
        let sig_hex = hex::encode(sig);
        let pk_hex = hex::encode(&self.validator_pk);

        let block = Block {
            height,
            parent: parent.hash.clone(),
            hash,
            timestamp,
            validator_pk: pk_hex,
            validator_sig: sig_hex,
            txs,
        };

        {
            let mut chain = self.chain.lock();
            chain.push(block.clone());
        }
        let _ = storage::append_block(&block);
        let _ = storage::snapshot_state(&self.ctx.lock(), block.height);

        block
    }

    pub fn last_block(&self) -> Block {
        self.chain.lock().last().cloned().expect("genesis must exist")
    }
    pub fn head(&self) -> Block { self.last_block() }
    pub fn block(&self, n: u64) -> Option<Block> { self.chain.lock().get(n as usize).cloned() }

    pub fn apply_external_block(&self, block: Block) -> Result<Block, String> {
        let parent = self.chain.lock().last().cloned().ok_or("no head")?;
        if block.parent != parent.hash { return Err("parent mismatch".into()); }
        if block.height != parent.height + 1 { return Err("height mismatch".into()); }

        for tx in &block.txs {
            let _ = self.dispatch_mut(&tx.call);
            self.inc_nonce(&tx.call.from);
        }

        {
            let mut chain = self.chain.lock();
            chain.push(block.clone());
        }
        let _ = storage::append_block(&block);
        let _ = storage::snapshot_state(&self.ctx.lock(), block.height);

        let committed: std::collections::HashSet<String> =
            block.txs.iter().map(|t| t.tx_hash.clone()).collect();
        {
            let mut pool = self.mempool.lock();
            pool.retain(|t| !committed.contains(&t.tx_hash));
        }

        Ok(block)
    }

    pub fn validator_info(&self) -> (String, String) {
        ("mldsa3".to_string(), hex::encode(&self.validator_pk))
    }
    pub fn validator_pk_hex(&self) -> String { hex::encode(&self.validator_pk) }
    pub fn has_sk(&self) -> bool { !self.validator_sk.is_empty() }

    pub fn next_nonce(&self, addr: &str) -> u64 { *self.nonces.lock().get(addr).unwrap_or(&0) }
    pub fn inc_nonce(&self, addr: &str) {
        let mut m = self.nonces.lock();
        let e = m.entry(addr.to_string()).or_insert(0);
        *e += 1;
    }

    pub fn dispatch_mut(&self, call: &Call) -> Result<serde_json::Value, crate::contracts::CtxError> {
        let mut ctx = self.ctx.lock();
        contracts::dispatch_mut(&mut ctx, &call.from, &call.contract, &call.method, &call.args)
    }

    pub fn dispatch_query(
        &self,
        contract: &str,
        method: &str,
        args: &serde_json::Value,
    ) -> Result<serde_json::Value, crate::contracts::CtxError> {
        let ctx = self.ctx.lock();
        contracts::dispatch_query(&ctx, contract, method, args)
    }

    pub fn submit(&self, call: Call, auth: Auth, nonce: u64, chain_id: String, tx_hash: String) {
        let max_pending: usize = std::env::var("QSC_MAX_PENDING_PER_ADDR").ok().and_then(|s| s.parse().ok()).unwrap_or(100);
        {
            let pool = self.mempool.lock();
            let count = pool.iter().filter(|tx| tx.call.from == call.from).count();
            if count >= max_pending {
                eprintln!("rate-limit: dropping tx from {} (pending={})", call.from, count);
                return;
            }
        }
        let tx = Tx { call, timestamp: now_ms(), auth, nonce, chain_id, tx_hash };
        self.mempool.lock().push(tx);
    }
}

pub fn new_shared() -> SharedRuntime {
    let chain_id = std::env::var("QSC_CHAIN_ID").unwrap_or_else(|_| "qsc-local".into());
    let (validator_sk, validator_pk) = match (std::env::var("QSC_VALIDATOR_SK"), std::env::var("QSC_VALIDATOR_PK")) {
        (Ok(sk_hex), Ok(pk_hex)) => (hex::decode(sk_hex.trim()).unwrap_or_default(), hex::decode(pk_hex.trim()).unwrap_or_default()),
        _ => pq::keygen_mldsa3(),
    };

    let rt = Arc::new(Runtime {
        ctx: Mutex::new(Ctx::default()),
        mempool: Mutex::new(Vec::new()),
        chain: Mutex::new(Vec::new()),
        nonces: Mutex::new(HashMap::new()),
        chain_id,
        validator_sk,
        validator_pk,
    });

    {
        let mut ctx = rt.ctx.lock();
        use std::sync::Arc as SyncArc;
        ctx.register(SyncArc::new(crate::contracts::token::Token));
    }

    {
        let mut chain = rt.chain.lock();
        if chain.is_empty() {
            let timestamp: u128 = std::env::var("QSC_GENESIS_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
                
            let parent = "0".repeat(128);
            let header_str = format!("{}|{}|{}|{}", parent, 0u64, 0usize, timestamp);
            let hash = hash_hex(header_str.as_bytes());
            let sig = pq::sign_mldsa3(hash.as_bytes(), &rt.validator_sk);
            let sig_hex = hex::encode(sig);
            let pk_hex = hex::encode(&rt.validator_pk);
            let genesis = Block { height: 0, parent, hash, timestamp, validator_pk: pk_hex, validator_sig: sig_hex, txs: vec![] };
            chain.push(genesis.clone());
            let _ = storage::append_block(&genesis);
            let _ = storage::snapshot_state(&rt.ctx.lock(), 0);
        }
    }

    rt
}
