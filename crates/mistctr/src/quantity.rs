use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceQuantity {
    Memory { bytes: u64 },
    Cpu { vcpu: u64 },
}

impl ResourceQuantity {
    pub fn as_memory(&self) -> Option<u64> {
        match self {
            Self::Memory { bytes } => Some(*bytes),
            _ => None,
        }
    }

    pub fn as_cpu(&self) -> Option<u64> {
        match self {
            Self::Memory { bytes } => Some(*bytes),
            _ => None,
        }
    }
}

#[derive(Debug, Error)]
pub enum QuantityParseError {
    #[error("Invalid quantity format.")]
    InvalidFormat,
    #[error("Invalid quantity unit.")]
    InvalidUnit,
}

impl TryFrom<&str> for ResourceQuantity {
    type Error = QuantityParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(QuantityParseError::InvalidFormat);
        }

        let mut chars = value.chars();
        let mut num_len = 0;
        let mut is_float = false;

        while let Some(c) = chars.next() {
            if !c.is_ascii_digit() && (c != '.' || c == '.' && is_float) {
                break;
            }

            if c == '.' {
                is_float = true;
            }

            num_len += 1;
        }

        if num_len == 0 {
            return Err(QuantityParseError::InvalidFormat);
        }

        let num: f64 = value[..num_len]
            .parse()
            .or(Err(QuantityParseError::InvalidFormat))?;

        let unit: DataUnit = value[num_len..]
            .try_into()
            .or(Err(QuantityParseError::InvalidUnit))?;

        Ok(Self::Memory {
            bytes: (unit.to_bytes() as f64 * num).round() as u64,
        })
    }
}

pub enum DataUnit {
    /// 1024 ^ n
    Kibibyte,
    Mebibyte,
    Gibibyte,
    Tebibyte,
    Pebibyte,
    Exbibyte,
    Zebibyte,
    Yobibyte,
    Robibyte,
    Quebibyte,

    /// 1000 ^ n
    Kilobyte,
    Megabyte,
    Gigabyte,
    Terabyte,
    Petabyte,
    Exabyte,
    Zettabyte,
    Yottabyte,
    Ronnabyte,
    Quettabyte,
}

impl TryFrom<&str> for DataUnit {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "k" => Ok(Self::Kilobyte),
            "M" => Ok(Self::Megabyte),
            "G" => Ok(Self::Gigabyte),
            "T" => Ok(Self::Terabyte),
            "P" => Ok(Self::Petabyte),
            "E" => Ok(Self::Exabyte),
            "Z" => Ok(Self::Zettabyte),
            "Y" => Ok(Self::Yottabyte),
            "R" => Ok(Self::Ronnabyte),
            "Q" => Ok(Self::Quettabyte),
            "Ki" => Ok(Self::Kibibyte),
            "Mi" => Ok(Self::Mebibyte),
            "Gi" => Ok(Self::Gibibyte),
            "Ti" => Ok(Self::Tebibyte),
            "Pi" => Ok(Self::Pebibyte),
            "Ei" => Ok(Self::Exbibyte),
            "Zi" => Ok(Self::Zebibyte),
            "Yi" => Ok(Self::Yobibyte),
            "Ri" => Ok(Self::Robibyte),
            "Qi" => Ok(Self::Quebibyte),
            _ => Err("Invalid unit."),
        }
    }
}

impl DataUnit {
    pub fn to_bytes(&self) -> u64 {
        match self {
            Self::Kilobyte => 1000,
            Self::Megabyte => 1000u64.pow(2),
            Self::Gigabyte => 1000u64.pow(3),
            Self::Terabyte => 1000u64.pow(4),
            Self::Petabyte => 1000u64.pow(5),
            Self::Exabyte => 1000u64.pow(6),
            Self::Zettabyte => 1000u64.pow(7),
            Self::Yottabyte => 1000u64.pow(8),
            Self::Ronnabyte => 1000u64.pow(9),
            Self::Quettabyte => 1000u64.pow(10),
            Self::Kibibyte => 1024,
            Self::Mebibyte => 1024u64.pow(2),
            Self::Gibibyte => 1024u64.pow(3),
            Self::Tebibyte => 1024u64.pow(4),
            Self::Pebibyte => 1024u64.pow(5),
            Self::Exbibyte => 1024u64.pow(6),
            Self::Zebibyte => 1024u64.pow(7),
            Self::Yobibyte => 1024u64.pow(8),
            Self::Robibyte => 1024u64.pow(9),
            Self::Quebibyte => 1024u64.pow(10),
        }
    }
}
