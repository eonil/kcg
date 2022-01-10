mod model;
mod scan;
mod lint;
mod codegen;
mod util;

use structopt::StructOpt;
use lint::Lint;
use codegen::CodeGen;

#[derive(StructOpt)]
#[structopt(name="kcg", about="A Schema Code-Gen.")]
struct Opt {
    /// Path to source OpenAPI 3.0 schema file.
    /// Please note that only certain subset will be supported.
    input: String,
    /// Path to write generated Rust code.
    /// KCG won't produce target code if this is not designated.
    /// Then effectively performs only lint stage.
    output: Option<String>,
}

fn main() {
    match run() {
        Err(x) => println!("{}", x),
        Ok(_) => (),
    }
}
fn run() -> Result<(),Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let src = std::fs::read_to_string(opt.input)?;
    let oas = serde_yaml::from_str::<scan::openapi3::model::Doc>(&src)?;

    // Lint.
    let mut x = lint::Context::default();
    oas.lint(lint::Path::default(), &mut x);
    println!("{}", x);
    x.check()?;

    match opt.output {
        None => (),
        Some(x) => {
            // Scan.
            let k = oas.scan()?;

            // Code-gen.
            let code = k.code();
            std::fs::write(x, code)?;
        },
    }
    Ok(())
}
