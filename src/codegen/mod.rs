pub mod rust;
// mod swift;
// mod typescript;

pub trait CodeGen {
    fn code(&self) -> String;
}
