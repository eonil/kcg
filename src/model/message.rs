//! Defines model of KCG.
//! 
//! KCG can build its own model by scanning other datastructure schema.
//! For OpenAPI 3.0 Schema, KCG find certain pattern to build certain types.
//! 
//! All types are `K` prefixed which means "Schema".

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Debug)]
pub enum KType {
    New(KNewType),
    Sum(KSumType),
    Prod(KProdType),
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KNewType {
    pub name: String,
    pub origin: KTypeRef,
    pub comment: String,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KSumType {
    pub name: String,
    pub variants: Vec<KSumTypeVariant>,
    pub comment: String,
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KSumTypeVariant {
    pub name: String,
    /// Type of stored data in this sum-type variant.
    /// Name-based sum-types can define array/optional content.
    /// Type-based sum-types only can define explicit reference to other type.
    pub content: KContentStorage,
    pub comment: String,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KProdType {
    pub name: String,
    pub fields: Vec<KProdTypeField>,
    pub comment: String,
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KProdTypeField {
    pub name: String,
    pub content: KContentStorage,
    pub comment: String,
}

/// An inveted concept to simplify type definition.
/// Proper support for optional/array types will require full support for generics.
/// To eliminate complexity of generics support, I just baked-in some essential generic patterns.
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KContentStorage {
    pub optional: bool,
    pub array: bool,
    pub r#type: KTypeRef,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Debug)]
pub enum KTypeRef {
    /// Unit type.
    /// Some code-gen can reject unit type.
    /// Unit type is implicitly defined by KCG.
    Unit,
    /// Pre-defined primitive types.
    /// Some code-gen can reject certain set of primitive types.
    /// Primitive types are implicitly defined by KCG.
    Prim(KPrimType),
    /// Name to a defined type.
    /// This must be a defined name in schema document.
    Def(String),
}
impl Default for KTypeRef {
    fn default() -> KTypeRef { Self::Unit }
}

/// A simple value with no substructure.
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Debug)]
pub enum KPrimType {
    Bool,
    I32,
    I64,
    F32,
    F64,
    String,
}