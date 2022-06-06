mod args;

use std::{fs::File, io::Write};

use clap::StructOpt;

use args::{ArgError, GenArgs, Measurement, MEASUREMENTS};

fn main() -> anyhow::Result<()> {
    let args = GenArgs::parse();

    let measurement: u64 = if let Some(measurement) = args.measurement.map(|x| x.to_lowercase()) {
        match measurement.as_str() {
            "byte" | "b" => Ok(Measurement::Byte),
            "kilobyte" | "kb" | "k" => Ok(Measurement::Kilobyte),
            "megabyte" | "mb" | "m" => Ok(Measurement::Megabyte),
            "gigabyte" | "gb" | "g" => Ok(Measurement::Gigabyte),
            _ => Err(ArgError::InvalidMeasurement(measurement)),
        }
    } else {
        Ok(Measurement::Byte)
    }?
    .into();

    let mut size: u64 = args.size as u64 * measurement;

    let cwd = std::env::current_dir()?;
    let path = cwd.join(args.path.unwrap_or_else(|| "file.txt".into()));

    let mut file = File::create(path)?;

    for measurement in MEASUREMENTS.iter().rev() {
        let num: u64 = size;
        size -= num;

        let bytes = measurement.into_bytes();
        for _ in 0..(num * u64::from(*measurement)) {
            file.write_all(&bytes)?;
        }
    }

    Ok(())
}
