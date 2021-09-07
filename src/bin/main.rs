use std::env;

use baikal::conversion_table::ConversionTable;
use baikal::prelude::*;
use clap::Clap;
use kalk::parser;

#[derive(Clap, Debug)]
#[clap(name = "baikal", author, about, version)]
struct Opt {
    /// Arithmetic expression
    #[clap(default_value = "56 gb / 6 + 4kib * 5 -4 mb + 4 B")]
    expr: String,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Show only minimal output
    #[clap(short, long)]
    minimal: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opt = Opt::parse_from(args);

    let replaced = ConversionTable::replace_units(opt.expr.clone());

    println!("input: {:?}", opt.expr);
    println!("interpreted: {:#?}", &replaced);

    let mut parser_context = parser::Context::new();
    let result = parser::eval(&mut parser_context, &replaced)
        .map_err(crate::Error::CalculationError)
        .unwrap();

    if let Some(outcome) = result {
        let table = ConversionTable::from_bytes(outcome.to_f64() as u128);
        println!("{}", table);
    }
}
