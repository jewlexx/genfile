use std::{error::Error, fmt::Display};

use clap::Parser;

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
}

pub const MEASUREMENTS: [Measurement; 4] = [
    Measurement::Byte,
    Measurement::Kilobyte,
    Measurement::Megabyte,
    Measurement::Gigabyte,
];

#[derive(Debug)]
pub enum ArgError {
    InvalidMeasurement(String),
}

impl Error for ArgError {}

impl Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgError::InvalidMeasurement(measurement) => write!(
                f,
                "Invalid Measurement: {}. Must be kb, mb, or gb",
                measurement,
            ),
        }
    }
}

impl From<Measurement> for u128 {
    fn from(measurement: Measurement) -> u128 {
        1024_u128.pow(measurement as u32)
    }
}

impl Measurement {
    pub fn into_bytes(self) -> Vec<u8> {
        let size: u128 = self.into();

        vec![0u8; size as usize]
    }
}

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
