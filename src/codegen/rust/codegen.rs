use extend::ext;
use indoc::formatdoc;
use crate::model::*;
use crate::codegen::*;

impl CodeGen for Doc1 {
    fn code(&self) -> String {
        // TODO: Implement `funcs` code-gen.
        formatdoc!("            
            {types}
        ",
        types=self.types.code())
    }
}
impl CodeGen for message::KType {
    fn code(&self) -> String {
        use message::KType::*;
        match self {
            New(x) => x.code(),
            Sum(x) => x.code(),
            Prod(x) => x.code(),
        }
    }
}
impl CodeGen for message::KNewType {
    fn code(&self) -> String {
        formatdoc!("
            pub type {name} = {origin};
        ",
        name=self.name,
        origin=self.origin.code())
    }
}
impl CodeGen for message::KSumType {
    fn code(&self) -> String {
        formatdoc!("
            {comment}
            #[derive(Serialize,Deserialize)]
            #[derive(Eq,PartialEq)]
            #[derive(Debug)]
            pub enum {name} {{
            {variants}
            }}
        ",
        comment=self.comment.code_documentation(),
        name=self.name,
        variants=self.variants.code().indent())
    }
}
impl CodeGen for message::KSumTypeVariant {
    fn code(&self) -> String {
        formatdoc!("
            {comment}
            {name}({type}),
        ",
        comment=self.comment.code_documentation(),
        name=self.name,
        type=self.content.code())
    }
}

impl CodeGen for message::KProdType {
    fn code(&self) -> String {
        formatdoc!("
            {comment}
            #[derive(Serialize,Deserialize)]
            #[derive(Eq,PartialEq)]
            #[derive(Debug)]
            pub struct {name} {{
            {fields}
            }}
        ",
        comment=self.comment.code_documentation(),
        name=self.name,
        fields=self.fields.code().indent())
    }
}
impl CodeGen for message::KProdTypeField {
    fn code(&self) -> String {
        formatdoc!("
            {comment}
            pub {name}: {type},
        ",
        comment=self.comment.code_documentation(),
        name=self.name,
        type=self.content.code())
    }
}
impl CodeGen for message::KContentStorage {
    fn code(&self) -> String {
        match (self.array, self.optional) {
            (false,false) => self.r#type.code(),
            (true,false) => format!("Vec<{name}>", name=self.r#type.code()),
            (false,true) => format!("Option<{name}>", name=self.r#type.code()),
            (true,true) => format!("Option<Vec<{name}>>", name=self.r#type.code()),
        }
    }
}

impl CodeGen for message::KTypeRef {
    fn code(&self) -> String {
        use message::KTypeRef::*;
        match self {
            Unit => "()".to_string(),
            Prim(x) => x.code(),
            Def(x) => x.clone(),
        }
    }
}
impl CodeGen for message::KPrimType {
    fn code(&self) -> String {
        use message::KPrimType::*;
        match self {
            Bool => "bool",
            I32 => "i32",
            I64 => "i64",
            F32 => "f32",
            F64 => "f64",
            String => "String",
        }
        .to_string()
    }
}




impl<T:CodeGen> CodeGen for Vec<T> {
    fn code(&self) -> String {
        self.iter().map(|x| x.code().trim().to_string()).collect::<Vec<String>>().join("\n")
    }
}

#[ext(name=StringUtil)] 
impl String {
    fn indent(&self) -> String {
        self.prefix("    ")
    }
    fn code_documentation(&self) -> String {
        if self.is_empty() { return self.clone() }
        self.prefix("/// ")
    }
    fn prefix(&self, prefix: &str) -> String {
        self.split("\n").map(|line| {
            let mut x = String::from(prefix);
            x.push_str(line);
            x
        }).collect::<Vec<String>>().join("\n")
    }
}