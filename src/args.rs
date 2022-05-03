use std::{error::Error, fmt::Display};

use argh::FromArgs;

pub enum Measurement {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
}

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

#[derive(FromArgs)]
#[argh(description = "A simple demo file generator")]
pub struct GenArgs {
    #[argh(positional)]
    pub size: usize,

    #[argh(option, short = 'm', description = "the data measurement to write")]
    pub measurement: Option<String>,

    #[argh(option, short = 'p', description = "the file to write to")]
    pub path: Option<String>,
}

impl GenArgs {
    pub fn parse() -> Self {
        argh::from_env()
    }
}
