use clap::Parser;

use crate::error::ArgError;

use crate::bytes::Bytes;

#[derive(Debug, Clone, Copy)]
pub enum Measurement {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
}

impl Measurement {
    pub fn from_arg(arg: String) -> Result<Self, ArgError> {
        match arg.as_str() {
            "byte" | "b" => Ok(Self::Byte),
            "kilobyte" | "kb" | "k" => Ok(Self::Kilobyte),
            "megabyte" | "mb" | "m" => Ok(Self::Megabyte),
            "gigabyte" | "gb" | "g" => Ok(Self::Gigabyte),
            _ => Err(ArgError::InvalidMeasurement(arg)),
        }
    }

    pub fn into_bytes(self) -> Bytes {
        Bytes::from(self)
    }
}

pub const MEASUREMENTS: [Measurement; 4] = [
    Measurement::Byte,
    Measurement::Kilobyte,
    Measurement::Megabyte,
    Measurement::Gigabyte,
];

#[derive(Parser)]
#[clap(version, author, about)]
pub struct GenArgs {
    /// The size of the final file
    pub size: u128,

    /// The data measurement to write
    #[clap(long, short)]
    pub measurement: Option<String>,

    /// The file to write to
    #[clap(long, short)]
    pub path: Option<String>,
}
