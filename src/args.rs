use std::str::FromStr;

use clap::Parser;

use crate::error::ArgError;

#[derive(Debug, Default, Clone, Copy)]
pub enum Measurement {
    #[default]
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
}

impl FromStr for Measurement {
    type Err = ArgError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "byte" | "b" => Ok(Self::Byte),
            "kilobyte" | "kb" | "k" => Ok(Self::Kilobyte),
            "megabyte" | "mb" | "m" => Ok(Self::Megabyte),
            "gigabyte" | "gb" | "g" => Ok(Self::Gigabyte),
            _ => Err(ArgError::InvalidMeasurement(s.to_string())),
        }
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
    #[clap(long, short, default_value_t = Measurement::Byte)]
    pub measurement: Measurement,

    /// The file to write to
    #[clap(long, short)]
    pub path: Option<String>,
}
