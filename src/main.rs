mod util;
mod types;
mod contracts;
mod runtime;
mod storage;
mod pq;
mod security;
mod consensus;

use actix_web::{App, HttpServer, get, post, web, Responder, HttpResponse};
use actix_web::rt::{spawn, time};
use runtime::{new_shared, SharedRuntime};
use types::RpcCall;

#[get("/head")]
async fn head(rt: web::Data<SharedRuntime>) -> impl Responder {
    web::Json(rt.head())
}

#[get("/block/{n}")]
async fn block(rt: web::Data<SharedRuntime>, path: web::Path<u64>) -> impl Responder {
    match rt.block(path.into_inner()) {
        Some(b) => HttpResponse::Ok().json(b),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/validator")]
async fn validator(rt: web::Data<SharedRuntime>) -> impl Responder {
    let (alg, pk) = rt.validator_info();
    web::Json(serde_json::json!({ "alg": alg, "validator_pk": pk }))
}

#[get("/chain")]
async fn chain(rt: web::Data<SharedRuntime>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "chain_id": rt.chain_id,
        "hash_alg": match crate::util::current_hash_alg() { crate::util::HashAlg::Blake2b512 => "blake2b-512", crate::util::HashAlg::Sha3_512 => "sha3-512" },
        "sig_algs_allowed": crate::security::allowed_sig_algs(),
        "validator": { "alg": "mldsa3", "pk": rt.validator_info().1 }
    }))
}

#[get("/nonce/{addr}")]
async fn nonce(rt: web::Data<SharedRuntime>, path: web::Path<String>) -> impl Responder {
    let n = rt.next_nonce(&path.into_inner());
    HttpResponse::Ok().json(serde_json::json!({ "next_nonce": n }))
}

#[post("/canonical")]
async fn canonical(body: web::Json<RpcCall>) -> impl Responder {
    let payload = serde_json::json!({
        "from": body.from,
        "nonce": body.nonce,
        "chain_id": body.chain_id,
        "contract": body.contract,
        "method": body.method,
        "args": body.args,
    });
    let s = serde_json::to_string(&payload).unwrap();
    HttpResponse::Ok().json(serde_json::json!({ "payload": s }))
}

#[post("/call")]
async fn call(rt: web::Data<SharedRuntime>, body: web::Json<RpcCall>) -> impl Responder {
    let pk_bytes = hex::decode(&body.pk).unwrap_or_default();
    let derived_addr = pq::address_from_pk(&pk_bytes);
    if body.from != derived_addr {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error":"from does not match addr(pk)",
            "expected": derived_addr
        }));
    }

    let allowed = crate::security::allowed_sig_algs();
    if !allowed.iter().any(|a| a == &body.alg.to_lowercase()) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error":"signature algorithm not allowed",
            "allowed": allowed
        }));
    }

    if body.chain_id != rt.chain_id {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error":"wrong chain_id",
            "expected": rt.chain_id
        }));
    }
    let expected = rt.next_nonce(&body.from);
    if body.nonce != expected {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error":"bad nonce",
            "expected": expected
        }));
    }

    let payload = serde_json::json!({
        "from": body.from,
        "nonce": body.nonce,
        "chain_id": body.chain_id,
        "contract": body.contract,
        "method": body.method,
        "args": body.args,
    });
    let payload_bytes = serde_json::to_vec(&payload).unwrap();

    let ok = match body.alg.as_str() {
        "mldsa3" => {
            let sig = hex::decode(&body.sig).unwrap_or_default();
            pq::verify_mldsa3(&payload_bytes, &sig, &pk_bytes)
        }
        _ => false
    };
    if !ok {
        return HttpResponse::BadRequest().json(serde_json::json!({"error":"invalid PQ signature"}));
    }

    if body.contract == "token" && body.method == "mint" {
        let (_alg, vpk_hex) = rt.validator_info();
        let vpk = hex::decode(&vpk_hex).unwrap_or_default();
        let default_minter = pq::address_from_pk(&vpk);
        let env_minter = std::env::var("QSC_MINTER_ADDR").ok();
        let minter = env_minter.as_deref().unwrap_or(&default_minter);
        if body.from != minter {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error":"mint not allowed for this sender",
                "required_minter": minter
            }));
        }
    }

    let tx_hash = crate::util::hash_hex(&payload_bytes);

    rt.submit(types::Call {
        from: body.from.clone(),
        contract: body.contract.clone(),
        method: body.method.clone(),
        args: body.args.clone(),
    }, types::Auth {
        alg: body.alg.clone(),
        pk: body.pk.clone(),
        sig: body.sig.clone(),
    }, body.nonce, body.chain_id.clone(), tx_hash);

    HttpResponse::Ok().json(serde_json::json!({
        "status":"enqueued",
        "will_apply_in_next_block": true
    }))
}

#[get("/query")]
async fn query(rt: web::Data<SharedRuntime>, q: web::Query<std::collections::HashMap<String,String>>) -> impl Responder {
    let contract = q.get("contract").cloned().unwrap_or_default();
    let method = q.get("method").cloned().unwrap_or_default();
    let args = q.get("args").map(|s| serde_json::from_str::<serde_json::Value>(s).unwrap_or(serde_json::json!({})))
        .unwrap_or(serde_json::json!({}));
    match rt.dispatch_query(&contract, &method, &args) {
        Ok(v) => HttpResponse::Ok().json(serde_json::json!({"ok": true, "result": v})),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({"ok": false, "error": format!("{:?}", e)})),
    }
}

#[get("/consensus/config")]
async fn consensus_config() -> impl Responder {
    match crate::consensus::PoAConfig::from_env() {
        Some(cfg) => HttpResponse::Ok().json(serde_json::json!({
            "validators": cfg.validators,
            "slot_ms": cfg.slot_ms
        })),
        None => HttpResponse::BadRequest().json(serde_json::json!({"error":"no PoA config (set QSC_VALIDATORS_JSON)"}))
    }
}

#[post("/consensus/commit")]
async fn consensus_commit(rt: web::Data<SharedRuntime>, body: web::Json<types::Block>) -> impl Responder {
    let cfg = match crate::consensus::PoAConfig::from_env() {
        Some(c) => c, None => return HttpResponse::BadRequest().json(serde_json::json!({"error":"no PoA config"}))
    };
    let parent = rt.last_block();
    if let Err(e) = crate::consensus::verify_block_poa(&cfg, &parent.hash, &body) {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": e}));
    }
    match rt.apply_external_block(body.into_inner()) {
        Ok(b) => HttpResponse::Ok().json(serde_json::json!({"ok": true, "height": b.height})),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({"ok": false, "error": e}))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rt = new_shared();
    let rt_bg = rt.clone();

    let consensus_mode = std::env::var("QSC_CONSENSUS").unwrap_or_else(|_| "local".into());
    if consensus_mode.to_lowercase() == "poa" {
        let rt_loop = rt.clone();
        spawn(async move {
            let cfg = match crate::consensus::PoAConfig::from_env() { Some(c) => c, None => return };
            let client = reqwest::Client::new();
            loop {
                time::sleep(std::time::Duration::from_millis(cfg.slot_ms)).await;
                let head_blk = rt_loop.last_block();
                let next_h = head_blk.height + 1;
                let expected = cfg.expected_leader(next_h);
                if expected.pk.eq_ignore_ascii_case(&rt_loop.validator_pk_hex()) && rt_loop.has_sk() {
                    let blk = rt_loop.produce_block();
                    for v in &cfg.validators {
                        if v.pk.eq_ignore_ascii_case(&rt_loop.validator_pk_hex()) { continue; }
                        let url = format!("{}/consensus/commit", v.url.trim_end_matches('/'));
                        let _ = client.post(&url).json(&blk).send().await;
                    }
                }
            }
        });
    } else {
        spawn(async move {
            loop {
                time::sleep(std::time::Duration::from_secs(3)).await;
                let b = rt_bg.produce_block();
                println!("Produced block {} (txs: {})", b.height, b.txs.len());
            }
        });
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(rt.clone()))
            .service(head)
            .service(block)
            .service(validator)
            .service(chain)
            .service(nonce)
            .service(canonical)
            .service(call)
            .service(query)
            .service(consensus_config)
            .service(consensus_commit)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
