use super::{Ctx, Contract, CtxResult, CtxError};
use serde_json::{json, Value};

pub struct Token;

fn add_u64(a: u64, b: u64) -> Result<u64, CtxError> {
    a.checked_add(b).ok_or_else(|| CtxError::Logic("overflow".into()))
}
fn sub_u64(a: u64, b: u64) -> Result<u64, CtxError> {
    a.checked_sub(b).ok_or_else(|| CtxError::Logic("underflow".into()))
}
fn is_valid_addr(s: &str) -> bool {
    s.len() == 128 && s.chars().all(|c| c.is_ascii_hexdigit())
}

impl Contract for Token {
    fn name(&self) -> &'static str { "token" }

    fn call(&self, ctx: &mut Ctx, caller: &str, method: &str, args: &Value) -> CtxResult {
        let ns = ctx.ns_mut(self.name());
        match method {
            "mint" => {
                let to = args.get("to").and_then(|v| v.as_str()).ok_or_else(|| CtxError::BadArgs("to".into()))?;
                if !is_valid_addr(to) { return Err(CtxError::BadArgs("to (invalid addr)".into())); }
                let amount = args.get("amount").and_then(|v| v.as_u64()).ok_or_else(|| CtxError::BadArgs("amount".into()))?;
                if amount == 0 { return Err(CtxError::BadArgs("amount must be > 0".into())); }

                let cap: u64 = std::env::var("QSC_TOKEN_MAX_SUPPLY").ok().and_then(|s| s.parse().ok()).unwrap_or(u64::MAX);
                let total = ns.get("total_supply").and_then(|v| v.as_u64()).unwrap_or(0);
                let new_total = add_u64(total, amount)?;
                if new_total > cap { return Err(CtxError::Logic("max supply exceeded".into())); }
                ns.insert("total_supply".into(), json!(new_total));

                let bal = ns.get(to).and_then(|v| v.as_u64()).unwrap_or(0);
                ns.insert(to.into(), json!(add_u64(bal, amount)?));
                Ok(json!({"ok": true, "event":"Mint","to":to,"amount":amount}))
            }
            "transfer" => {
                let to = args.get("to").and_then(|v| v.as_str()).ok_or_else(|| CtxError::BadArgs("to".into()))?;
                if !is_valid_addr(to) { return Err(CtxError::BadArgs("to (invalid addr)".into())); }
                if to == caller { return Err(CtxError::Logic("self-transfer not allowed".into())); }
                let amount = args.get("amount").and_then(|v| v.as_u64()).ok_or_else(|| CtxError::BadArgs("amount".into()))?;
                if amount == 0 { return Err(CtxError::BadArgs("amount must be > 0".into())); }

                let from_bal = ns.get(caller).and_then(|v| v.as_u64()).unwrap_or(0);
                let new_from = sub_u64(from_bal, amount)?;
                ns.insert(caller.into(), json!(new_from));

                let to_bal = ns.get(to).and_then(|v| v.as_u64()).unwrap_or(0);
                ns.insert(to.into(), json!(add_u64(to_bal, amount)?));
                Ok(json!({"ok": true, "event":"Transfer","from":caller,"to":to,"amount":amount}))
            }
            _ => Err(CtxError::MethodNotFound),
        }
    }

    fn query(&self, ctx: &Ctx, method: &str, args: &Value) -> CtxResult {
        let ns = ctx.ns(self.name());
        match method {
            "total_supply" => {
                let v = ns.and_then(|m| m.get("total_supply")).cloned().unwrap_or(json!(0));
                Ok(v)
            }
            "balance_of" => {
                let who = args.get("who").and_then(|v| v.as_str()).ok_or_else(|| CtxError::BadArgs("who".into()))?;
                let v = ns.and_then(|m| m.get(who)).cloned().unwrap_or(json!(0));
                Ok(v)
            }
            _ => Err(CtxError::MethodNotFound),
        }
    }
}
