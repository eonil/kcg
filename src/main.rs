mod model;
mod scan;
mod lint;
mod codegen;
mod util;

fn main() {
    use codegen::*;
    let x = model::Doc1::default();
    let s = x.code();
    println!("{}", s);
}
