#![cfg(test)]

use indoc::indoc;
use crate::model::message::*;
use crate::codegen::CodeGen;

#[test]
fn sum_type_code() {
    let a = KType::Sum(KSumType { 
        name: "Pet".to_string(), 
        variants: vec![
            KSumTypeVariant {
                name: "Cat".to_string(),
                content: KContentStorage {
                    optional: false,
                    array: false,
                    r#type: KTypeRef::Def("Cat".to_string()),
                },
                comment: "".to_string(),
            },
            KSumTypeVariant {
                name: "Dog".to_string(),
                content: KContentStorage {
                    optional: false,
                    array: false,
                    r#type: KTypeRef::Def("Dog".to_string()),
                },
                comment: "".to_string(),
            },
        ], 
        comment: "".to_string() });
    let b = a.code();
    assert_eq!(b.trim(), indoc!("
        #[derive(Serialize,Deserialize)]
        #[derive(Eq,PartialEq)]
        #[derive(Debug)]
        pub enum Pet {
            Cat(Cat),
            Dog(Dog),
        }
    ").trim());
}

#[test]
fn prod_type_code() {
    let a = KType::Prod(KProdType { 
        name: "Ship".to_string(), 
        fields: vec![
            KProdTypeField {
                name: "fuel".to_string(),
                content: KContentStorage {
                    optional: true,
                    array: false,
                    r#type: KTypeRef::Prim(KPrimType::Bool),
                },
                comment: "".to_string(),
            },
            KProdTypeField {
                name: "cargo".to_string(),
                content: KContentStorage {
                    optional: false,
                    array: false,
                    r#type: KTypeRef::Def("Cargo".to_string()),
                },
                comment: "".to_string(),
            },
            KProdTypeField {
                name: "crews".to_string(),
                content: KContentStorage {
                    optional: false,
                    array: true,
                    r#type: KTypeRef::Prim(KPrimType::String),
                },
                comment: "".to_string(),
            },
        ],
        comment: "".to_string() });
    let b = a.code();
    assert_eq!(b.trim(), indoc!("
        #[derive(Serialize,Deserialize)]
        #[derive(Eq,PartialEq)]
        #[derive(Debug)]
        pub struct Ship {
            pub fuel: Option<bool>,
            pub cargo: Cargo,
            pub crews: Vec<String>,
        }
    ").trim());
}
