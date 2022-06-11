use std::{fs::File, io::Write};

mod args;
mod bytes;
mod error;

use args::{GenArgs, Measurement, MEASUREMENTS};
use bytes::{Bytes, BytesArray};

fn main() -> anyhow::Result<()> {
    let args: GenArgs = clap::Parser::parse();

    let measurement = if let Some(measurement) = args.measurement {
        Measurement::from_arg(measurement.to_lowercase())?
    } else {
        Measurement::Byte
    };

    let bytes_size = Bytes::from(measurement);

    let mut size: u128 = args.size * bytes_size.0;

    let cwd = std::env::current_dir()?;
    let path = cwd.join(args.path.unwrap_or_else(|| "file.txt".into()));

    let mut file = File::create(path)?;

    'outer: for measurement in MEASUREMENTS.iter().rev() {
        loop {
            let bytes_size = Bytes::from(*measurement);

            if size < bytes_size.0 {
                println!("sadge");
                continue 'outer;
            }

            // Yo I am confused about this too dw I am working on figuring it out
            let num = size / bytes_size.0;
            size -= num;

            let num = num / bytes_size.0;

            let bytes = BytesArray::from(*measurement);
            for _ in 0..(num * bytes_size.0) {
                file.write_all(&bytes.0)?;
            }

            if size == 0 {
                break 'outer;
            }
        }
    }

    Ok(())
}
