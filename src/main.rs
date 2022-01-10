mod model;
mod scan;
mod lint;
mod codegen;
mod util;

use structopt::StructOpt;
use lint::Lint;

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

    /// Prefix code for generated code.
    #[structopt(long="include")]
    prelude: Option<String>,
    /// Skipping type names.
    /// KCG won't make code for types with names in `skippings`.
    /// You are supposed to provide type definitions yourself using <prelude> option.
    #[structopt(long="skip")]
    skippings: Vec<String>,
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
            let k = oas.scan(lint::Path::default())?;

            // Code-gen.
            let mut code = String::new();
            code.push_str(&read_file_or_default(opt.prelude)?);
            code.push_str("\n\n");
            code.push_str(&k.code(&opt.skippings));
            std::fs::write(&x, code)?;
        },
    }
    Ok(())
}

fn read_file_or_default(path:Option<String>) -> Result<String,Box<dyn std::error::Error>> {
    match path {
        None => Ok(String::default()),
        Some(x) => Ok(std::fs::read_to_string(&x)?),
    }
}