use std::str::FromStr;

use clap::ArgEnum;

use crate::error::Error;

#[allow(non_camel_case_types)]
#[derive(ArgEnum, Debug, PartialEq, Eq)]
#[clap(rename_all = "verbatim")]
pub enum BinaryPrefix {
    B,
    KiB,
    MiB,
    GiB,
    TiB,
    kB,
    MB,
    GB,
    TB,
}

impl FromStr for BinaryPrefix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        clap::ArgEnum::from_str(s, true).map_err(Error::UnitParseError)
    }
}