use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub type CtxResult = Result<Value, CtxError>;

#[derive(Debug)]
pub enum CtxError {
    BadArgs(String),
    MethodNotFound,
    ContractNotFound,
    Logic(String),
}

pub trait Contract: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn call(&self, ctx: &mut Ctx, caller: &str, method: &str, args: &Value) -> CtxResult;
    fn query(&self, ctx: &Ctx, method: &str, args: &Value) -> CtxResult;
}

#[derive(Default)]
pub struct Ctx {
    state: HashMap<String, HashMap<String, Value>>,
    contracts: HashMap<&'static str, Arc<dyn Contract>>,
}

impl Ctx {
    pub fn register(&mut self, c: Arc<dyn Contract>) {
        self.contracts.insert(c.name(), c);
    }
    pub fn ns_mut(&mut self, ns: &str) -> &mut HashMap<String, Value> {
        self.state.entry(ns.into()).or_default()
    }
    pub fn ns(&self, ns: &str) -> Option<&HashMap<String, Value>> {
        self.state.get(ns)
    }
    pub fn state_map(&self) -> &HashMap<String, HashMap<String, Value>> {
        &self.state
    }
}

pub fn dispatch_mut(ctx: &mut Ctx, caller: &str, contract: &str, method: &str, args: &Value) -> CtxResult {
    if let Some(c) = ctx.contracts.get(contract).cloned() {
        c.call(ctx, caller, method, args)
    } else {
        Err(CtxError::ContractNotFound)
    }
}
pub fn dispatch_query(ctx: &Ctx, contract: &str, method: &str, args: &Value) -> CtxResult {
    if let Some(c) = ctx.contracts.get(contract).cloned() {
        c.query(ctx, method, args)
    } else {
        Err(CtxError::ContractNotFound)
    }
}

pub mod token;
