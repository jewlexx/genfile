mod args;

use std::{fs::File, io::Write};

use clap::StructOpt;

use args::{GenArgs, Measurement, MEASUREMENTS};

fn main() -> anyhow::Result<()> {
    let args = GenArgs::parse();

    let measurement: u128 = if let Some(measurement) = args.measurement {
        Measurement::from_arg(measurement.to_lowercase())?
    } else {
        Measurement::Byte
    }
    .into();

    let mut size: u128 = args.size * measurement;

    let cwd = std::env::current_dir()?;
    let path = cwd.join(args.path.unwrap_or_else(|| "file.txt".into()));

    let mut file = File::create(path)?;

    for measurement in MEASUREMENTS.iter().rev() {
        let num = size;
        size -= num;

        let bytes = measurement.into_bytes();
        for _ in 0..(num * u128::from(*measurement)) {
            file.write_all(&bytes)?;
        }
    }

    Ok(())
}
