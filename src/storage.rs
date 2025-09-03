use crate::types::Block;
use std::fs::{OpenOptions, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

fn data_dir() -> PathBuf {
    let p = std::env::var("QSC_DATA_DIR").unwrap_or_else(|_| "./data".into());
    let pb = PathBuf::from(p);
    let _ = create_dir_all(&pb);
    pb
}

pub fn append_block(b: &Block) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .create(true).append(true)
        .open(data_dir().join("chain.jsonl"))?;
    let line = serde_json::to_string(b).unwrap();
    writeln!(f, "{}", line)?;
    Ok(())
}

pub fn snapshot_state(ctx: &crate::contracts::Ctx, height: u64) -> std::io::Result<()> {
    let mut f = File::create(data_dir().join("state.json"))?;
    let s = serde_json::to_string_pretty(&ctx.state_map())?;
    f.write_all(s.as_bytes())?;
    let mut pf = File::create(data_dir().join("params.json"))?;
    let params = serde_json::json!({
        "hash_alg": match crate::util::current_hash_alg() { crate::util::HashAlg::Blake2b512 => "blake2b-512", crate::util::HashAlg::Sha3_512 => "sha3-512" },
        "sig_algs_allowed": crate::security::allowed_sig_algs(),
        "chain_id": std::env::var("QSC_CHAIN_ID").unwrap_or_else(|_| "qsc-local".into()),
        "height": height
    });
    pf.write_all(serde_json::to_string_pretty(&params).unwrap().as_bytes())?;
    Ok(())
}
