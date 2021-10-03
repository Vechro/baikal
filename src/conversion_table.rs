use std::lazy::SyncLazy;

use either::Either;
use regex::{Captures, Regex};

use crate::binary_prefix::BinaryPrefix;

type HybridNum = Either<u128, f64>;

static BINARY_RE: SyncLazy<Regex> = SyncLazy::new(|| {
    Regex::new("(?P<amt>[0-9]+)[\\r\\n\\t\\f\\v ]*(?i)(?P<unit>b|kib|mib|gib|tib|kb|mb|gb|tb)")
        .unwrap()
});

#[derive(Debug)]
pub struct ConversionTable {
    /// Bytes
    pub b: u128,
    // IEC standard (base-2)
    pub kib: HybridNum,
    pub mib: HybridNum,
    pub gib: HybridNum,
    pub tib: HybridNum,
    // SI standard (base-10)
    pub kb: HybridNum,
    pub mb: HybridNum,
    pub gb: HybridNum,
    pub tb: HybridNum,
}

impl ConversionTable {
    pub fn replace_units(s: String) -> String {
        let replaced = BINARY_RE.replace_all(&s, |caps: &Captures| Self::convert_units(caps));

        replaced.to_string()
    }

    fn convert_units(caps: &Captures) -> impl AsRef<str> {
        let amount = match caps.name("amt") {
            Some(amount) => amount.as_str().parse::<u128>().unwrap(),
            None => return String::new(),
        };

        let units = caps
            .name("unit")
            .map_or(Ok(BinaryPrefix::B), |m| m.as_str().parse::<BinaryPrefix>())
            .unwrap();

        // This is stupid inefficient I can only pray this is optimized away
        let conversion_table = Self::from_bytes(amount, units);

        conversion_table.b.to_string()
    }

    pub fn from_bytes(x: u128, original_b_prefix: BinaryPrefix) -> Self {
        let b = match original_b_prefix {
            BinaryPrefix::B => x,
            BinaryPrefix::KiB => x * (1 << 10),
            BinaryPrefix::MiB => x * (1 << 20),
            BinaryPrefix::GiB => x * (1 << 30),
            BinaryPrefix::TiB => x * (1 << 40),
            BinaryPrefix::kB => x * 1_000,
            BinaryPrefix::MB => x * 1_000_000,
            BinaryPrefix::GB => x * 1_000_000_000,
            BinaryPrefix::TB => x * 1_000_000_000_000,
        };

        fn go(b: u128, original: &BinaryPrefix, comparison: BinaryPrefix) -> HybridNum {
            match *original == comparison {
                true => HybridNum::Left(b),
                false => {
                    let b_f = b as f64;
                    let f = match comparison {
                        BinaryPrefix::B => unreachable!(),
                        BinaryPrefix::KiB => b_f / 1024f64,
                        BinaryPrefix::MiB => b_f / 1048576f64,
                        BinaryPrefix::GiB => b_f / 1073741824f64,
                        BinaryPrefix::TiB => b_f / 1099511627776f64,
                        BinaryPrefix::kB => b_f / 1_000f64,
                        BinaryPrefix::MB => b_f / 1_000_000f64,
                        BinaryPrefix::GB => b_f / 1_000_000_000f64,
                        BinaryPrefix::TB => b_f / 1_000_000_000_000f64,
                    };
                    HybridNum::Right(f)
                }
            }
        }

        Self {
            b,
            kib: go(b, &original_b_prefix, BinaryPrefix::KiB),
            mib: go(b, &original_b_prefix, BinaryPrefix::MiB),
            gib: go(b, &original_b_prefix, BinaryPrefix::GiB),
            tib: go(b, &original_b_prefix, BinaryPrefix::TiB),
            kb: go(b, &original_b_prefix, BinaryPrefix::kB),
            mb: go(b, &original_b_prefix, BinaryPrefix::MB),
            gb: go(b, &original_b_prefix, BinaryPrefix::GB),
            tb: go(b, &original_b_prefix, BinaryPrefix::TB),
        }
    }
}

impl std::fmt::Display for ConversionTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PRECISION: usize = 4;

        use colored::Colorize;
        writeln!(f, "≡ {:>16} B", self.b)?;
        writeln!(f, "    {}", "IEC standard (base-2)".bold())?;
        writeln!(f, "≈ {:>16.1$} KiB", self.kib, PRECISION)?;
        writeln!(f, "≈ {:>16.1$} MiB", self.mib, PRECISION)?;
        writeln!(f, "≈ {:>16.1$} GiB", self.gib, PRECISION)?;
        writeln!(f, "≈ {:>16.1$} TiB", self.tib, PRECISION)?;
        writeln!(f, "    {}", "SI standard (base-10)".bold())?;
        writeln!(f, "≈ {:>16.1$} kB", self.kb, PRECISION)?;
        writeln!(f, "≈ {:>16.1$} MB", self.mb, PRECISION)?;
        writeln!(f, "≈ {:>16.1$} GB", self.gb, PRECISION)?;
        writeln!(f, "≈ {:>16.1$} TB", self.tb, PRECISION)
    }
}
