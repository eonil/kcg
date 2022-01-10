use super::model as oa;
use crate::lint::*;

impl Lint for oa::Doc {
    /// Checks whether current OAS document exactly fits to KCG model.
    /// Results will be recorded into `context`.
    fn lint<'a>(self: &Self, path: Path, context: &mut Context) {
        match &self.components {
            None => context.error(path.appending("components"), "missing required property `components`"),
            Some(x) => x.lint(path.appending("components"), context),
        };
    }
}

impl Lint for oa::Components {
    fn lint<'a>(self: &Self, path: Path, context: &mut Context) {
        match &self.schemas {
            None => context.error(path.appending("schemas"), "missing required property `schemas`"),
            Some(x) => {
                for (name,schema) in x {
                    schema.lint(path.appending(&name), context);
                }
            },
        }
    }
}

impl Lint for oa::ReferencedOrInlineSchema {
    fn lint<'a>(self: &Self, path: Path, context: &mut Context) {
        use oa::ReferencedOrInlineSchema::*;
        match &self {
            Referenced(x) => x.lint(path.clone(), context),
            Inline(x) => x.lint(path.clone(), context),
        }
    }
}

impl Lint for oa::Schema {
    fn lint<'a>(self: &Self, path: Path, context: &mut Context) {
        // A Schema object defines a type.
        // KCG accepts only certain pattern of Schema object that are mapped to KCG types.
        // Everything else will be rejected.
        context.guard_nil_property_for_unsupported_feature(&self.any_of, path.appending("anyOf"), "property for unsupported feature has non-`nil` value");
        context.guard_nil_property_for_unsupported_feature(&self.format, path.appending("format"), "property for unsupported feature has non-`nil` value");
        context.guard_nil_property_for_unsupported_feature(&self.not, path.appending("not"), "property for unsupported feature has non-`nil` value");
        let det = self.is_new_type(path.clone(), context)
             || self.is_sum_type(path.clone(), context)
             || self.is_prod_type(path.clone(), context);
        if !det {
            context.error(path.clone(), "unknown/unsupported pattern of schema (none of new-type/sum-type/prod-type)");
        }
    }
}
impl oa::Schema {
    fn is_new_type(self: &Self, path: Path, context: &mut Context) -> bool {
        let x = self.r#type.as_ref().map(|x| &x[..]).unwrap_or("");
        match x {
            "boolean" | "number" | "string" => (),
            _ => return false,
        };
        // Now this is a new-type.
        context.guard_nil_property(&self.items, path.appending("items"), "must be `nil` to make new-type");
        context.guard_nil_property(&self.all_of, path.appending("allOf"), "must be `nil` to make new-type");
        context.guard_nil_property(&self.format, path.appending("format"), "must be `nil` to make new-type");
        context.guard_nil_property(&self.any_of, path.appending("anyOf"), "must be `nil` to make new-type");
        context.guard_nil_property(&self.properties, path.appending("properties"), "must be `nil` to make new-type");
        context.guard_nil_property(&self.r#enum, path.appending("enum"), "must be `nil` to make new-type");
        true
    }
    fn is_sum_type(self: &Self, path: Path, context: &mut Context) -> bool {
        if !(self.r#type.as_ref().map(|x| x == "object").unwrap_or(false)) { return false }
        if self.one_of == None { return false }
        // Now this is a sum-type.
        context.guard_nil_property(&self.items, path.appending("items"), "must be `nil` to make sum-type");
        context.guard_nil_property(&self.format, path.appending("allOf"), "must be `nil` to make sum-type");
        context.guard_nil_property(&self.any_of, path.appending("anyOf"), "must be `nil` to make sum-type");
        context.guard_nil_property(&self.properties, path.appending("properties"), "must be `nil` to make sum-type");
        context.guard_nil_property(&self.r#enum, path.appending("enum"), "must be `nil` to make sum-type");
        context.guard_some_property(&self.one_of, path.appending("oneOf"), "must be non-`nil` to make sum-type");
        true
    }
    fn is_prod_type(self: &Self, path: Path, context: &mut Context) -> bool {
        if !(self.r#type.as_ref().map(|x| x == "object").unwrap_or(false)) { return false }
        if self.properties == None { return false }
        // Now this is a prod-type.
        context.guard_nil_property(&self.items, path.appending("items"), "must be non-`nil` to make prod-type");
        context.guard_nil_property(&self.format, path.appending("allOf"), "must be non-`nil` to make prod-type");
        context.guard_nil_property(&self.any_of, path.appending("anyOf"), "must be non-`nil` to make prod-type");
        context.guard_nil_property(&self.one_of, path.appending("oneOf"), "must be non-`nil` to make prod-type");
        context.guard_nil_property(&self.discriminator, path.appending("discriminator"), "must be non-`nil` to make prod-type");
        context.guard_nil_property(&self.r#enum, path.appending("enum"), "must be non-`nil` to make prod-type");
        context.guard_some_property(&self.properties, path.appending("properties"), "must be non-`nil` to make prod-type");
        true
    }
}

impl Lint for oa::Reference {
    fn lint<'a>(self: &Self, path: Path, context: &mut Context) {
        let x = &self.r#ref;
        context.guard(x.starts_with("#/"), path.appending("$ref"), "JSON Schema reference expression must starts with `#/`");
        context.guard(!x.ends_with("/"), path.appending("$ref"), "JSON Schema reference expression must starts with `#/`");
        context.guard(x.starts_with("#/components/schemas/"), path.appending("$ref"), "KCG supports only `#/components/schemas` prefixed reference");
        context.guard(x.split_at("#/components/schemas/".len()).1.contains("/") == false, path.appending("$ref"), "KCG does not support nested paths");
        
    }
}






impl Context {
    fn guard_nil_property_for_unsupported_feature<T>(self: &mut Self, property: &Option<T>, path: Path, message: &'static str) {
        match property {
            None => (),
            Some(_) => self.error(path, message),
        }   
    }
    fn guard_nil_property<T>(self: &mut Self, property: &Option<T>, path: Path, message: &'static str) {
        match property {
            None => (),
            Some(_) => self.error(path, message),
        }       
    }
    fn guard_some_property<T>(self: &mut Self, property: &Option<T>, path: Path, message: &'static str) {
        match property {
            None => self.error(path, message),
            Some(_) => (),
        }       
    }
    fn guard(self: &mut Self, condition: bool, path: Path, message: &'static str) {
        if !condition { self.error(path, message) }
    }
}