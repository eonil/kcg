use extend::ext;
use crate::model::Doc1;
use crate::model::message::*;
use super::model as oa;

pub type Result<T> = std::result::Result<T,String>;

impl oa::Doc {
    pub fn scan(self: &oa::Doc) -> Result<Doc1> {
        let mut k = Doc1::default();
        for comps in self.components.iter() {
            k.types.extend(comps.scan_types()?);
        }
        Ok(k)
    }
}
impl oa::Components {
    fn scan_types(&self) -> Result<Vec<KType>> {
        use oa::ReferencedOrInlineSchema::*;
        let mut z = Vec::new();
        for schemas in self.schemas.iter() {
            for (name,x) in schemas {
                z.push(match x {
                    Referenced(_) => return err(""),
                    Inline(k) => k.scan_type(name)?,
                });
            }
        }
        Ok(z)
    }
}
impl oa::ReferencedOrInlineSchema {
    fn scan_type_ref(&self) -> Result<KTypeRef> {
        use oa::ReferencedOrInlineSchema::*;
        Ok(match self {
            Referenced(x) => KTypeRef::Def(x.r#ref.clone()),
            Inline(x) => KTypeRef::Prim(x.scan_prim_type()?),
        })
    }
}
impl oa::Schema {
    fn scan_type(&self, name: &str) -> Result<KType> {
        let x = match (self.r#type.str(), &self.one_of) {
            ("string", _) => KType::New(self.scan_new_type(name)?),
            ("object", Some(_)) => KType::Sum(self.scan_sum_type(name)?),
            ("object", None) => KType::Prod(self.scan_prod_type(name)?),
            _ => return err("unknown/unsupported schema pattern (none of new/sum/prod type)"),
        };
        Ok(x)
    }
    fn scan_new_type(&self, name: &str) -> Result<KNewType> {
        if self.r#type.str() != "string" { return err("new-type must be JSON String form (we do not support non-string new-types)") }
        Ok(KNewType {
            name: name.to_string(),
            origin: KTypeRef::Prim(self.scan_prim_type()?), 
            comment: self.scan_composed_comment(),
        })
    }
    fn scan_sum_type(&self, name: &str) -> Result<KSumType> {
        if self.r#type.str() != "object" { return err("sum-type must be JSON Object form") }
        Ok(KSumType {
            name: name.to_string(),
            variants: self.scna_sum_type_variants()?,
            comment: self.scan_composed_comment(),
        })
    }
    fn scna_sum_type_variants(&self) -> Result<Vec<KSumTypeVariant>> {
        type KK = oa::ReferencedOrInlineSchema;
        use oa::ReferencedOrInlineSchema::*;
        let subschemas = self.one_of.guard("cannot scan sum-type variant from schema with no `oneOf` defined")?;
        if !(subschemas.iter().all(KK::is_referenced) || subschemas.iter().all(KK::is_inline)) { 
            return err("subnodes of `oneOf` node must be all reference or all inline to be a KCG sum-type");
        }
        let mut z = Vec::<KSumTypeVariant>::new();
        for k in subschemas.iter() {
            match k {
                Inline(x) => {
                    // Type-A sum-type. Name-based variants.
                    if !(x.r#type.str() != "object") { return err("sum-type variant must be a JSON Object type in OpenAPI schema") }
                    let reqs = x.required.guard("name-based sum-type variant node's properties must be all required")?;
                    let props = x.properties.guard("name-based sum-type variant node must have 1 property")?;
                    for req in reqs {
                        if !props.contains_key(req) { return err("name-based sum-type variant node's properties must be all required") }
                    }
                    if props.len() != 1 { return err("name-based sum-type variant node must have 1 property") }
                    for (name,prop) in props {
                        match prop {
                            Inline(_) => return err("name-based sum-type variant's inline property must be a reference to an explicitly named type"),
                            Referenced(x) => {
                                z.push(KSumTypeVariant {
                                    name: x.scan_referenced_type_name().to_string(),
                                    content: KContentStorage { optional: false, array: false, r#type: KTypeRef::Def(x.scan_referenced_type_name().to_string()) },
                                    comment: String::new(),
                                });
                            },
                        }
                        break;
                    }
                },
                Referenced(x) => {
                    // Type-B sum-type. Type-based variants.
                    z.push(KSumTypeVariant {
                        name: x.scan_referenced_type_name().to_string(),
                        content: KContentStorage { optional: false, array: false, r#type: KTypeRef::Def(x.scan_referenced_type_name().to_string()) },
                        comment: String::new(),
                    });
                },
            }
        }
        Ok(z)
    }
    fn scan_prod_type(&self, name: &str) -> Result<KProdType> {
        let z = KProdType {
            name: name.to_string(),
            fields: self.scan_prod_type_fields()?,
            comment: self.scan_composed_comment(),
        };
        Ok(z)
    }
    fn scan_prod_type_fields(&self) -> Result<Vec<KProdTypeField>> {
        use oa::ReferencedOrInlineSchema::*;
        let props = self.properties.guard("prod-type must have `properties` property")?;
        let mut z = Vec::new();
        for (name,prop) in props {
            let optional = self.required.as_ref().map(|x|!x.contains(name)).unwrap_or(true);
            match prop {
                // Referenced(_) => return err("property node must be an OAS inline Schema object and cannot be a reference"),
                Referenced(x) => z.push(KProdTypeField {
                    name: name.to_string(),
                    content: KContentStorage {
                        optional: optional,
                        array: false,
                        r#type: KTypeRef::Def(x.scan_referenced_type_name().to_string()),
                    },
                    comment: "".to_string(),
                }),
                Inline(x) => z.push(KProdTypeField {
                    name: name.to_string(),
                    content: x.scan_content_type(name, optional)?,
                    comment: x.scan_composed_comment(),
                }),
            }
        }
        Ok(z)
    }
    fn scan_composed_comment(&self) -> String {
        let a = self.title.str();
        let b = self.summary.str();
        let c = self.description.str();
        let mut z = String::new();
        if a != "" { 
            z.push_str(a);
            z.push_str("\n\n");
        }
        if b != "" { 
            z.push_str(b);
            z.push_str("\n\n");
        }
        z.push_str(c);
        z
    }
    /// Scans prod-type field's type from a OAS property node.
    fn scan_content_type(&self, name:&str, optional:bool) -> Result<KContentStorage> {
        let z = match self.r#type.str() {
            // An array.
            "array" => {
                let x = self.items.guard("a JSON Array type OAS node must have a `items` property node")?;
                KContentStorage {
                    optional: optional,
                    array: true, 
                    r#type: x.scan_type_ref()?,
                }
            },
            // Inline type definitions are not allowed.
            // All types must be defined at document root with explicit names.
            "object" => return err("inline type definitions are now allowed (all types must be explicitly named)"),
            // A prim-type.
            _ => KContentStorage {
                optional: optional,
                array: false, 
                r#type: KTypeRef::Prim(self.scan_prim_type()?),
            },
        };
        Ok(z)
    }
    fn scan_prim_type(&self) -> Result<KPrimType> {
        use KPrimType::*;
        let x = match (self.r#type.str(), self.format.str()) {
            ("boolean","") => Bool,
            ("integer","int32") => I32,
            ("integer","int64") => I64,
            ("number","float") => F32,
            ("number","double") => F64,
            ("string","") => String,
            (_,_) => return err("unknown/unsupported type/format combination for KCG primitive type"),
        };
        Ok(x)
    }
}

impl oa::Reference {
    fn scan_referenced_type_name(&self) -> &str {
        self.r#ref.split("/").last().unwrap_or("")
    }
}








#[ext(name=ListUtil)]
impl oa::List<String> {
    fn contains(&self, s:&str) -> bool {
        for x in self.iter() {
            if x == s { return true }
        }
        false
    }
}

#[ext(name=OptionUtil)]
impl<T> Option<T> {
    fn guard(&self, message: &str) -> Result<&T> {
        match self {
            None => Err(String::from(message)),
            Some(x) => Ok(x),
        }
    }
}
#[ext(name=OptionStringUtil)]
impl Option<String> {
    fn str(&self) -> &str {
        match self {
            None => "",
            Some(x) => &*x,
        }
    }
}
impl oa::ReferencedOrInlineSchema {
    fn guard_inline(&self, message: &str) -> Result<&oa::Schema> {
        use oa::ReferencedOrInlineSchema::*;
        match self {
            Referenced(_) => err(message),
            Inline(x) => Ok(x),
        }
    }
}

fn err<T,S:ToString>(message:S) -> Result<T> {
    Err(message.to_string())
}
