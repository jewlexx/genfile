mod args;

use std::{fs::File, io::Write};

use args::{ArgError, GenArgs, Measurement};

fn main() -> anyhow::Result<()> {
    let args = GenArgs::parse();

    let measurement: u64 = if let Some(measurement) = args.measurement.map(|x| x.to_lowercase()) {
        match measurement.as_str() {
            "byte" | "b" => Ok(Measurement::Byte),
            "kilobyte" | "kb" | "k" => Ok(Measurement::Kilobyte),
            "megabyte" | "mb" | "m" => Ok(Measurement::Megabyte),
            "gigabyte" | "gb" | "g" => Ok(Measurement::Gigabyte),
            _ => Err(ArgError::InvalidMeasurement(measurement.to_string())),
        }
    } else {
        Ok(Measurement::Byte)
    }?
    .into();

    let mut size: u64 = args.size as u64 * measurement;

    let cwd = std::env::current_dir()?;
    let path = cwd.join(args.path.unwrap_or_else(|| "file.txt".into()));

    let mut file = File::create(path)?;

    if size > u64::from(Measurement::Gigabyte) {
        let gigs: u64 = size / u64::from(Measurement::Gigabyte);
        size -= gigs * u64::from(Measurement::Gigabyte);

        let bytes = Measurement::Gigabyte.into_bytes();
        for _ in 0..gigs {
            file.write_all(&bytes)?;
        }
    }

    if size > u64::from(Measurement::Megabyte) {
        let megas: u64 = size / u64::from(Measurement::Megabyte);
        size -= megas * u64::from(Measurement::Megabyte);

        let bytes = Measurement::Megabyte.into_bytes();
        for _ in 0..megas {
            file.write_all(&bytes)?;
        }
    }

    if size > u64::from(Measurement::Kilobyte) {
        let kilos: u64 = size / u64::from(Measurement::Kilobyte);
        size -= kilos * u64::from(Measurement::Kilobyte);

        let bytes = Measurement::Kilobyte.into_bytes();
        for _ in 0..kilos {
            file.write_all(&bytes)?;
        }
    }

    if size > 0 {
        let bytes = Measurement::Byte.into_bytes();
        for _ in 0..size {
            file.write_all(&bytes)?;
        }
    }

    Ok(())
}
