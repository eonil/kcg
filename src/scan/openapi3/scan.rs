use extend::ext;
use crate::model::Doc1;
use crate::model::message::*;
use crate::lint;
use super::model as oa;

pub type Result<T> = std::result::Result<T,String>;

impl oa::Doc {
    pub fn scan(self: &oa::Doc, path: lint::Path) -> Result<Doc1> {
        let mut k = Doc1::default();
        for comps in self.components.iter() {
            k.types.extend(comps.scan_types(path.appending("components"))?);
        }
        Ok(k)
    }
}
impl oa::Components {
    fn scan_types(&self, path: lint::Path) -> Result<Vec<KType>> {
        use oa::ReferencedOrInlineSchema::*;
        let mut z = Vec::new();
        for schemas in self.schemas.iter() {
            for (name,x) in schemas {
                z.push(match x {
                    Referenced(_) => return err(&path, "schemas in components must be defined inline"),
                    Inline(k) => k.scan_type(path.appending("schemas").appending(name), name)?,
                });
            }
        }
        Ok(z)
    }
}
impl oa::ReferencedOrInlineSchema {
    fn scan_type_ref(&self, path: lint::Path) -> Result<KTypeRef> {
        use oa::ReferencedOrInlineSchema::*;
        Ok(match self {
            Referenced(x) => KTypeRef::Def(x.r#ref.clone()),
            Inline(x) => KTypeRef::Prim(x.scan_prim_type(path)?),
        })
    }
}
impl oa::Schema {
    fn scan_type(&self, path: lint::Path, name: &str) -> Result<KType> {
        let x = match (self.r#type.str(), self.is_prim_type(), &self.one_of, &self.r#enum) {
            (_, true, None, None) => KType::New(self.scan_new_type(path, name)?),
            (_, true, None, Some(_)) => KType::Enum(self.scan_enum_type(path, name)?),
            ("object", false, Some(_), None) => KType::Sum(self.scan_sum_type(path, name)?),
            ("object", false, None, None) => KType::Prod(self.scan_prod_type(path, name)?),
            _ => return err(&path, "unknown/unsupported schema pattern (none of new/enum/sum/prod type)"),
        };
        Ok(x)
    }
    fn scan_new_type(&self, path: lint::Path, name: &str) -> Result<KNewType> {
        if self.r#type.str() != "string" { return err(&path, "new-type must be JSON String form (we do not support non-string new-types)") }
        Ok(KNewType {
            name: name.to_string(),
            origin: KTypeRef::Prim(self.scan_prim_type(path.clone())?), 
            comment: self.scan_composed_comment(path.clone()),
        })
    }
    fn scan_enum_type(&self, path: lint::Path, name: &str) -> Result<KEnumType> {
        if self.r#type.str() != "string" { return err(&path, "enum-type must be JSON String form (we do not support non-string new-types)") }
        let mut cases = Vec::new(); 
        for xx in self.r#enum.iter() {
            for x in xx {
                let case = match x {
                    serde_json::Value::String(case) => case,
                    _ => return err(&path, "enum-type case must be JSON String type (no support for other types)"),
                };
                cases.push(KEnumTypeCase {
                    name: case.to_string(),
                    comment: "".to_string(),
                });
            }
            break;
        }
        Ok(KEnumType {
            name: name.to_string(),
            cases: cases,
            comment: self.scan_composed_comment(path),
        })
    }
    fn scan_sum_type(&self, path: lint::Path, name: &str) -> Result<KSumType> {
        if self.r#type.str() != "object" { return err(&path, "sum-type must be JSON Object form") }
        Ok(KSumType {
            name: name.to_string(),
            variants: self.scan_sum_type_variants(path.clone())?,
            comment: self.scan_composed_comment(path.clone()),
        })
    }
    fn scan_sum_type_variants(&self, path: lint::Path) -> Result<Vec<KSumTypeVariant>> {
        type KK = oa::ReferencedOrInlineSchema;
        use oa::ReferencedOrInlineSchema::*;
        let subschemas = self.one_of.guard(&path, "cannot scan sum-type variant from schema with no `oneOf` defined")?;
        if !(subschemas.iter().all(KK::is_referenced) || subschemas.iter().all(KK::is_inline)) { 
            return err(&path, "subnodes of `oneOf` node must be all reference or all inline to be a KCG sum-type");
        }
        let mut z = Vec::<KSumTypeVariant>::new();
        for k in subschemas.iter() {
            match k {
                Inline(x) => {
                    // Type-A sum-type. Name-based variants.
                    if !(x.r#type.str() != "object") { return err(&path, "sum-type variant must be a JSON Object type in OpenAPI schema") }
                    let reqs = x.required.guard(&path, "name-based sum-type variant node's properties must be all required")?;
                    let props = x.properties.guard(&path, "name-based sum-type variant node must have 1 property")?;
                    for req in reqs {
                        if !props.contains_key(req) { return err(&path, "name-based sum-type variant node's properties must be all required") }
                    }
                    if props.len() != 1 { return err(&path, "name-based sum-type variant node must have 1 property") }
                    for (name,prop) in props {
                        let subpath = path.appending("oneOf").appending(name);
                        match prop {
                            Inline(_) => return err(&path, "name-based sum-type variant's inline property must be a reference to an explicitly named type"),
                            Referenced(x) => {
                                z.push(KSumTypeVariant {
                                    name: x.scan_referenced_type_name(subpath.clone()).to_string(),
                                    content: KContentStorage { optional: false, array: false, r#type: KTypeRef::Def(x.scan_referenced_type_name(subpath.clone()).to_string()) },
                                    comment: String::new(),
                                });
                            },
                        }
                        break;
                    }
                },
                Referenced(x) => {
                    let subpath = path.appending("oneOf");
                    // Type-B sum-type. Type-based variants.
                    z.push(KSumTypeVariant {
                        name: x.scan_referenced_type_name(subpath.clone()).to_string(),
                        content: KContentStorage { optional: false, array: false, r#type: KTypeRef::Def(x.scan_referenced_type_name(subpath.clone()).to_string()) },
                        comment: String::new(),
                    });
                },
            }
        }
        Ok(z)
    }
    fn scan_prod_type(&self, path: lint::Path, name: &str) -> Result<KProdType> {
        let z = KProdType {
            name: name.to_string(),
            fields: self.scan_prod_type_fields(path.clone())?,
            comment: self.scan_composed_comment(path.clone()),
        };
        Ok(z)
    }
    fn scan_prod_type_fields(&self, path: lint::Path) -> Result<Vec<KProdTypeField>> {
        use oa::ReferencedOrInlineSchema::*;
        let props = self.properties.guard(&path, "prod-type must have `properties` property")?;
        let mut z = Vec::new();
        for (name,prop) in props {
            let optional = self.required.as_ref().map(|x|!x.contains(name)).unwrap_or(true);
            let subpath = path.appending("properties").appending(name);
            match prop {
                // Referenced(_) => return err("property node must be an OAS inline Schema object and cannot be a reference"),
                Referenced(x) => z.push(KProdTypeField {
                    name: name.to_string(),
                    content: KContentStorage {
                        optional: optional,
                        array: false,
                        r#type: KTypeRef::Def(x.scan_referenced_type_name(subpath).to_string()),
                    },
                    comment: "".to_string(),
                }),
                Inline(x) => z.push(KProdTypeField {
                    name: name.to_string(),
                    content: x.scan_content_type(subpath.clone(), optional)?,
                    comment: x.scan_composed_comment(subpath.clone()),
                }),
            }
        }
        Ok(z)
    }
    fn scan_composed_comment(&self, _path: lint::Path) -> String {
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
    fn scan_content_type(&self, path: lint::Path, optional:bool) -> Result<KContentStorage> {
        let z = match self.r#type.str() {
            // An array.
            "array" => {
                let x = self.items.guard(&path, "a JSON Array type OAS node must have a `items` property node")?;
                KContentStorage {
                    optional: optional,
                    array: true, 
                    r#type: x.scan_type_ref(path.appending("items"))?,
                }
            },
            // Inline type definitions are not allowed.
            // All types must be defined at document root with explicit names.
            "object" => return err(&path, "inline type definitions are now allowed (all types must be explicitly named)"),
            // A prim-type.
            _ => KContentStorage {
                optional: optional,
                array: false, 
                r#type: KTypeRef::Prim(self.scan_prim_type(path)?),
            },
        };
        Ok(z)
    }
    fn scan_prim_type(&self, path: lint::Path) -> Result<KPrimType> {
        use KPrimType::*;
        let x = match (self.r#type.str(), self.format.str()) {
            ("boolean","") => Bool,
            ("integer","int32") => I32,
            ("integer","int64") => I64,
            ("number","float") => F32,
            ("number","double") => F64,
            ("string","") => String,
            (_,_) => return err(&path, "unknown/unsupported type/format combination for KCG primitive type"),
        };
        Ok(x)
    }
}

impl oa::Reference {
    fn scan_referenced_type_name(&self, _path: lint::Path) -> &str {
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
    fn guard(&self, path: &lint::Path, message: &str) -> Result<&T> {
        match self {
            None => err(path, message),
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

fn err<T,S:ToString>(path: &lint::Path, message:S) -> Result<T> {
    Err(format!("openapi3::scan({}): {}", path, message.to_string()))
}
