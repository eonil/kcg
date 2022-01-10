#![cfg(test)]

use indoc::indoc;
use crate::lint::*;
use crate::model::Doc1;
use crate::model::message::*;
use super::lint::*;
use super::model::*;
use super::scan;

#[test]
fn test_reading_openapi_spec() -> Result<(), serde_yaml::Error> {
    let a = indoc!(r#"
        openapi: 3.0.1
        info:
            title: Swagger Petstore
            version: 1.2.3
        paths: {}
        components:
            schemas:
                Order:
                    type: object
                    properties:
                        id: { type: integer }
                        quantity: { type: integer }
    "#);
    let b = serde_yaml::from_str::<Doc>(a)?;
    assert_eq!(b.openapi, "3.0.1");
    assert_eq!(b.info.version, Some("1.2.3".to_string()));
    Ok(())
}

#[test]
#[should_panic]
fn test_reading_lint_fail() {
    let a = indoc!(r#"
        openapi: 3.0.1
        info:
            title: Swagger Petstore
            version: 1.2.3
        paths: {}
        components:
            schemas:
                Order:
                    type: object 
    "#);
    let b = serde_yaml::from_str::<Doc>(a).unwrap();
    let mut x = Context::default();
    b.lint(Path::default(), &mut x);
    println!("{}", x);
    x.check().unwrap();
}

#[test]
fn test_reading_new_type_pass() -> Result<(), Box<dyn std::error::Error>> {
    let a = indoc!(r#"
        openapi: 3.0.1
        info:
            title: Swagger Petstore
            version: 1.2.3
        paths: {}
        components:
            schemas:
                Order: { type: string }
    "#);
    let b = serde_yaml::from_str::<Doc>(a)?;
    
    let mut x = Context::default();
    b.lint(Path::default(), &mut x);
    println!("{}", x);
    x.check()?;

    let c = b.scan(Path::default())?;
    assert_eq!(c.types.len(), 1);
    assert_eq!(c.types[0], KType::New(KNewType { 
        name: "Order".to_string(), 
        origin: KTypeRef::Prim(KPrimType::String), 
        comment: String::new() }));
    Ok(())
}

#[test]
fn test_reading_enum_type_pass() -> Result<(), Box<dyn std::error::Error>> {
    let a = indoc!(r#"
        openapi: 3.0.1
        info:
            title: Swagger Petstore
            version: 1.2.3
        paths: {}
        components:
            schemas:
                Fish: 
                    type: string
                    enum: [Whale, Shrimp]

    "#);
    let b = serde_yaml::from_str::<Doc>(a)?;
    
    let mut x = Context::default();
    b.lint(Path::default(), &mut x);
    println!("{}", x);
    x.check()?;

    let c = b.scan(Path::default())?;
    assert_eq!(c.types.len(), 1);
    assert_eq!(c.types[0], KType::Enum(KEnumType {
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
    }));
    Ok(())
}

#[test]
fn test_reading_sum_type_pass() -> Result<(), Box<dyn std::error::Error>> {
    let a = indoc!(r#"
        openapi: 3.0.1
        info:
            title: Swagger Petstore
            version: 1.2.3
        paths: {}
        components:
            schemas:
                Pet: 
                    type: object
                    oneOf:
                        - $ref: '#/components/schemas/Cat'
                        - $ref: '#/components/schemas/Dog'
                Cat: { type: string }
                Dog: { type: string }
    "#);
    let b = serde_yaml::from_str::<Doc>(a)?;
    
    let mut x = Context::default();
    b.lint(Path::default(), &mut x);
    println!("{}", x);
    x.check()?;

    let c = b.scan(Path::default())?;
    assert_eq!(c.types.len(), 3);
    println!("{:#?}", c);
    assert_eq!(c.types[0], KType::Sum(KSumType { 
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
        comment: "".to_string() }));
    Ok(())
}

#[test]
fn test_reading_prod_type_pass() -> Result<(), Box<dyn std::error::Error>> {
    let a = indoc!(r#"
        openapi: 3.0.1
        info:
            title: Swagger Petstore
            version: 1.2.3
        paths: {}
        components:
            schemas:
                Ship: 
                    type: object
                    required: [cargo, crews]
                    properties: 
                        fuel:
                            type: boolean
                        cargo:
                            $ref: '#/components/schemas/Cargo'
                        crews:
                            type: array 
                            items: 
                                type: string
                Cargo: 
                    type: object
                    properties: {}
    "#);
    let b = serde_yaml::from_str::<Doc>(a)?;
    
    let mut x = Context::default();
    b.lint(Path::default(), &mut x);
    println!("{}", x);
    x.check()?;

    let c = b.scan(Path::default())?;
    assert_eq!(c.types.len(), 2);
    println!("{:#?}", c);
    assert_eq!(c.types[0], KType::Prod(KProdType { 
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
        comment: "".to_string() }));    
    Ok(())
}
