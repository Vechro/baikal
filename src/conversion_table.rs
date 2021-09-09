use std::lazy::SyncLazy;

use regex::{Captures, Regex};

use crate::binary_prefix::BinaryPrefix;

static BINARY_RE: SyncLazy<Regex> = SyncLazy::new(|| {
    Regex::new("(?P<amt>[0-9]+)[\\r\\n\\t\\f\\v ]*(?i)(?P<unit>b|kib|mib|gib|tib|kb|mb|gb|tb)")
        .unwrap()
});

#[derive(Debug)]
pub struct ConversionTable {
    /// Bytes
    pub b: u128,
    // IEC standard (base-2)
    pub kib: u128,
    pub mib: u128,
    pub gib: u128,
    pub tib: u128,
    // SI standard (base-10)
    pub kb: u128,
    pub mb: u128,
    pub gb: u128,
    pub tb: u128,
}

impl From<(u128, BinaryPrefix)> for ConversionTable {
    fn from((amount, unit): (u128, BinaryPrefix)) -> Self {
        match unit {
            BinaryPrefix::B => Self::from_bytes(amount),
            BinaryPrefix::KiB => Self::from_kibibytes(amount),
            BinaryPrefix::MiB => Self::from_mebibytes(amount),
            BinaryPrefix::GiB => Self::from_gibibytes(amount),
            BinaryPrefix::TiB => Self::from_tebibytes(amount),
            BinaryPrefix::kB => Self::from_kilobytes(amount),
            BinaryPrefix::MB => Self::from_megabytes(amount),
            BinaryPrefix::GB => Self::from_gigabytes(amount),
            BinaryPrefix::TB => Self::from_terabytes(amount),
        }
    }
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
        let conversion_table: Self = (amount, units).into();

        conversion_table.b.to_string()
    }

    pub fn from_bytes(x: u128) -> Self {
        Self {
            b: x,
            kib: x / (1 << 10),
            mib: x / (1 << 20),
            gib: x / (1 << 30),
            tib: x / (1 << 40),
            kb: x / 1_000,
            mb: x / 1_000_000,
            gb: x / 1_000_000_000,
            tb: x / 1_000_000_000_000,
        }
    }

    pub fn from_kibibytes(x: u128) -> Self {
        let b = x * (1 << 10);
        Self {
            b,
            kib: x,
            mib: x / (1 << 10),
            gib: x / (1 << 20),
            tib: x / (1 << 30),
            kb: x * b / 1_000,
            mb: x * b / 1_000_000,
            gb: x * b / 1_000_000_000,
            tb: x * b / 1_000_000_000_000,
        }
    }

    pub fn from_mebibytes(x: u128) -> Self {
        let b = x * (1 << 20);
        Self {
            b,
            kib: x * (1 << 10),
            mib: x,
            gib: x / (1 << 10),
            tib: x / (1 << 20),
            kb: x * b / 1_000,
            mb: x * b / 1_000_000,
            gb: x * b / 1_000_000_000,
            tb: x * b / 1_000_000_000_000,
        }
    }

    pub fn from_gibibytes(x: u128) -> Self {
        let b = x * (1 << 30);
        Self {
            b,
            kib: x * (1 << 20),
            mib: x * (1 << 10),
            gib: x,
            tib: x / (1 << 10),
            kb: x * b / 1_000,
            mb: x * b / 1_000_000,
            gb: x * b / 1_000_000_000,
            tb: x * b / 1_000_000_000_000,
        }
    }

    pub fn from_tebibytes(x: u128) -> Self {
        let b = x * (1 << 40);
        Self {
            b,
            kib: x * (1 << 30),
            mib: x * (1 << 20),
            gib: x * (1 << 10),
            tib: x,
            kb: x * b / 1_000,
            mb: x * b / 1_000_000,
            gb: x * b / 1_000_000_000,
            tb: x * b / 1_000_000_000_000,
        }
    }

    pub fn from_kilobytes(x: u128) -> Self {
        let b = x * 1_000;
        Self {
            b,
            kib: b / (1 << 10),
            mib: b / (1 << 20),
            gib: b / (1 << 30),
            tib: b / (1 << 40),
            kb: x,
            mb: x / 1_000,
            gb: x / 1_000_000,
            tb: x / 1_000_000_000,
        }
    }

    pub fn from_megabytes(x: u128) -> Self {
        let b = x * 1_000_000;
        Self {
            b,
            kib: b / (1 << 10),
            mib: b / (1 << 20),
            gib: b / (1 << 30),
            tib: b / (1 << 40),
            kb: x * 1_000,
            mb: x,
            gb: x / 1_000,
            tb: x / 1_000_000,
        }
    }

    pub fn from_gigabytes(x: u128) -> Self {
        let b = x * 1_000_000_000;
        Self {
            b,
            kib: b / (1 << 10),
            mib: b / (1 << 20),
            gib: b / (1 << 30),
            tib: b / (1 << 40),
            kb: x * 1_000_000,
            mb: x * 1_000,
            gb: x,
            tb: x / 1_000,
        }
    }

    pub fn from_terabytes(x: u128) -> Self {
        let b = x * 1_000_000_000_000;
        Self {
            b,
            kib: b / (1 << 10),
            mib: b / (1 << 20),
            gib: b / (1 << 30),
            tib: b / (1 << 40),
            kb: x * 1_000_000_000,
            mb: x * 1_000_000,
            gb: x * 1_000,
            tb: x,
        }
    }
}

impl std::fmt::Display for ConversionTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        writeln!(f, "≡ {:>16} B", self.b)?;
        writeln!(f, "    {}", "IEC standard (base-2)".bold())?;
        writeln!(f, "≈ {:>16} KiB", self.kib)?;
        writeln!(f, "≈ {:>16} MiB", self.mib)?;
        writeln!(f, "≈ {:>16} GiB", self.gib)?;
        writeln!(f, "≈ {:>16} TiB", self.tib)?;
        writeln!(f, "    {}", "SI standard (base-10)".bold())?;
        writeln!(f, "≈ {:>16} kB", self.kb)?;
        writeln!(f, "≈ {:>16} MB", self.mb)?;
        writeln!(f, "≈ {:>16} GB", self.gb)?;
        writeln!(f, "≈ {:>16} TB", self.tb)
    }
}
