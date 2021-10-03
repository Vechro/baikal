use std::env;

use baikal::{binary_prefix::BinaryPrefix, conversion_table::ConversionTable, error::Error};
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "baikal", author, about, version)]
struct Opt {
    /// Arithmetic expression
    expr: String,

    /// Show only minimal output
    #[clap(short, long)]
    minimal: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opt = Opt::parse_from(args);

    let replaced = ConversionTable::replace_units(opt.expr);

    let result = fasteval::ez_eval(&replaced, &mut fasteval::EmptyNamespace)
        .map_err(crate::Error::CalculationError)
        .unwrap();

    let table = ConversionTable::from_bytes(result as u128, BinaryPrefix::B);

    if opt.minimal {
        println!("{} B", table.b.to_string());
    } else {
        println!("{}", table);
    }
}

#[test]
fn replacing() {
    assert_eq!(
        ConversionTable::replace_units("56 gb / 6 + 4kib * 5 -4 mb + 4 B".to_string()),
        "56000000000 / 6 + 4096 * 5 -4000000 + 4"
    )
}
