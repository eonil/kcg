use serde_derive::{Serialize,Deserialize};
use super::message::*;

/// HTTP-based REST-like service.
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct HService {
    pub funcs: Vec<HFunc>,
    pub comment: String,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct HFunc {
    pub input: HFuncInput,
    pub output: HFuncOutput,
    pub comment: String,
}

/// Represents HTTP REST-like input.
/// - `body` must be set to KTypeRef::Unit` if `method == "GET".
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct HFuncInput {
    pub method: String,
    pub path: String,
    pub query: Vec<HFuncInputQuery>,
    pub body: KTypeRef,
    pub comment: String,
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Debug)]
pub struct HFuncInputQuery {
    pub name: String,
    pub r#type: KPrimType,
    pub comment: String,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct HFuncOutput {
    pub cases: Vec<HFuncCase>,
    pub comment: String,
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct HFuncCase {
    pub status: i32,
    pub body: KTypeRef,
    pub comment: String,
}