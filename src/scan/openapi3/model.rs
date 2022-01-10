//! OpenAPI Schema 3.0 Model
//! https://swagger.io/specification/
//! 
//! Strictly follows spec. Simplification will be done to KCG's own model.
//! Strict subset of OpenAPI 3.0 model. 
//! Unsupported features will cause an error.

use serde_derive::{Serialize, Deserialize};

pub type List<T> = std::vec::Vec<T>;
// pub type Map<K,V> = std::collections::HashMap<K,V>;
// pub type Map<K,V> = vector_map::VecMap<K,V>;
pub type Map<K,V> = linear_map::LinearMap<K,V>;

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Doc {
    pub openapi: String,
    pub info: Info,
    pub servers: Option<List<Server>>,
    pub paths: Paths,
    pub components: Option<Components>,
    pub security: Option<List<SecurityRequirement>>,
    pub tags: Option<List<Tag>>,
    pub external_docs: Option<ExternalDocumentation>,
}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Info {
    pub title: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: Option<String>,
}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct License {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    pub variables: Map<String,ServerVariable>,
}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct ServerVariable {
    pub r#enum: Option<List<String>>,
    pub default: String,
    pub description: Option<String>,
}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Paths {

}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Components {
    pub schemas: Option<Map<String,ReferencedOrInlineSchema>>,
}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct SecurityRequirement {

}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Tag {

}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct ExternalDocumentation {

}

/// OpenAPI 3.0 Schema object.
/// - Follows JSON Schema 2020-12 spec where needed
///   - https://json-schema.org/specification.html
/// - Only certain strict subset will be supported.
/// - Any unsupported properties for unsupported features will be rejected.
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all="camelCase")]
pub struct Schema {
    pub title:  Option<String>,
    pub summary: Option<String>,
    pub required: Option<List<String>>,
    pub r#enum: Option<List<serde_json::Value>>,

    pub r#type: Option<String>,
    
    pub all_of: Option<Vec<ReferencedOrInlineSchema>>,
    pub one_of: Option<Vec<ReferencedOrInlineSchema>>,
    pub any_of: Option<Vec<ReferencedOrInlineSchema>>,
    pub not: Option<Box<ReferencedOrInlineSchema>>,
    pub items: Option<Box<ReferencedOrInlineSchema>>,
    pub properties: Option<Map<String, ReferencedOrInlineSchema>>,
    #[serde(default)]
    pub additional_properties: Box<AdditionalProperties>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub default: Option<serde_json::Value>,

    pub discriminator: Option<Discriminator>,
    pub example: Option<serde_json::Value>,
    #[serde(default)]
    pub deprecated: bool,
}
impl Schema {
    pub fn is_prim_type(&self) -> bool {
        match self.r#type.as_ref() {
            Some(x) => match x.as_str() {
                "boolean" | "integer" | "number" | "string" => true,
                _ => false,
            },
            None => false,
        }
    }
}

/// When request bodies or response payloads may be one of a number of different schemas, a discriminator object can be used to aid in serialization, deserialization, and validation. The discriminator is a specific object in a schema which is used to inform the consumer of the specification of an alternative schema based on the value associated with it.
/// When using the discriminator, inline schemas will not be considered.
/// 
/// The discriminator object is legal only when using one of the composite keywords oneOf, anyOf, allOf.
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Discriminator {
    pub property_name: String,
    pub mapping: Option<Map<String,String>>,
}







#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(untagged)]
#[serde(rename_all="camelCase")]
pub enum ReferencedOrInlineSchema {
    Referenced(Reference),
    Inline(Schema),
}
impl ReferencedOrInlineSchema {
    pub fn is_referenced(&self) -> bool { 
        match self { Self::Referenced(_) => true, _ => false }
    }
    pub fn is_inline(&self) -> bool { 
        match self { Self::Inline(_) => true, _ => false }
    }
}

/// Only local paths are supported.
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Reference {
    #[serde(rename="$ref")]
    pub r#ref: String,
}

#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(untagged)]
#[serde(rename_all="camelCase")]
pub enum AdditionalProperties {
    Bool(bool),
    Referenced(Reference),
    Inline(Schema),
}
impl Default for AdditionalProperties {
    fn default() -> AdditionalProperties { Self::Bool(false) }
}













