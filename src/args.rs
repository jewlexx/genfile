use std::{error::Error, fmt::Display};

use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub enum Measurement {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
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

impl From<Measurement> for u64 {
    fn from(measurement: Measurement) -> u64 {
        let i = measurement as u8;

        1024_i32.pow(i as u32) as u64
    }
}

impl Measurement {
    pub fn into_bytes(self) -> Vec<u8> {
        let size: u64 = self.into();

        vec![0u8; size as usize]
    }
}

#[derive(Parser)]
#[clap(version, author, about)]
pub struct GenArgs {
    /// The size of the final file
    pub size: usize,

    /// The data measurement to write
    #[clap(long, short)]
    pub measurement: Option<String>,

    /// The file to write to
    #[clap(long, short)]
    pub path: Option<String>,
}
