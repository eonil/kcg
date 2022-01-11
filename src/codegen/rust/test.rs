#![cfg(test)]

use indoc::indoc;
use crate::model::message::*;
use crate::codegen::CodeGen;

#[test]
fn enum_type_code() {
    let a = KType::Enum(KEnumType {
        name: "Fish".to_string(),
        cases: vec![
            KEnumTypeCase {
                name: "Whale".to_string(),
                comment: "".to_string(),
            },
            KEnumTypeCase {
                name: "Shrimp".to_string(),
                comment: "".to_string(),
            },
        ],
        comment: "".to_string(),
    });
    let b = a.code();
    assert_eq!(b.trim(), indoc!(r#"
        #[derive(Serialize,Deserialize)]
        #[derive(Eq,PartialEq)]
        #[derive(Debug)]
        pub enum Fish {
            Whale,
            Shrimp,
        }
        impl std::str::FromStr for Fish {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use Fish::*;
                match s {
                    "Whale" => Ok(Whale),
                    "Shrimp" => Ok(Shrimp),
                    _ => Err("unknown case name".to_string()),
                }
            }
        }
        impl std::string::ToString for Fish {
            fn to_string(&self) -> String {
                use Fish::*;
                match self {
                    Whale => "Whale".to_string(),
                    Shrimp => "Shrimp".to_string(),
                }
            }
        }
    "#).trim());
}

#[test]
fn sum_type_code() {
    let a = KType::Sum(KSumType { 
        name: "Pet".to_string(), 
        discriminant: "type".to_string(),
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
    assert_eq!(b.trim(), indoc!(r#"
        #[derive(Serialize,Deserialize)]
        #[derive(Eq,PartialEq)]
        #[derive(Debug)]
        #[serde(tag="type")]
        pub enum Pet {
            Cat(Cat),
            Dog(Dog),
        }
    "#).trim());
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
