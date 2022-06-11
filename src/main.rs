use std::{fs::File, io::Write};

mod args;
mod bytes;
mod error;

use args::{GenArgs, Measurement, MEASUREMENTS};
use bytes::{Bytes, BytesArray};

fn write_measurement(
    quantity: u128,
    measurement: Measurement,
    file: &mut File,
) -> anyhow::Result<()> {
    let array = BytesArray::from(measurement);

    for _ in 0..quantity {
        file.write_all(&array.0)?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: GenArgs = clap::Parser::parse();

    let measurement = if let Some(measurement) = args.measurement {
        Measurement::from_arg(measurement.to_lowercase())?
    } else {
        Measurement::Byte
    };

    let size: u128 = args.size * measurement.into_bytes().0;

    let cwd = std::env::current_dir()?;
    let path = cwd.join(args.path.unwrap_or_else(|| "file.txt".into()));

    let gigabytes = size / Measurement::Gigabyte.into_bytes().0;
    let megabytes = size / Measurement::Megabyte.into_bytes().0 - gigabytes;
    let kilobytes = size / Measurement::Kilobyte.into_bytes().0 - gigabytes - megabytes;
    let bytes = size / Measurement::Byte.into_bytes().0 - gigabytes - megabytes - kilobytes;

    let mut file = File::create(path)?;

    write_measurement(gigabytes, Measurement::Gigabyte, &mut file)?;
    write_measurement(megabytes, Measurement::Megabyte, &mut file)?;
    write_measurement(kilobytes, Measurement::Kilobyte, &mut file)?;
    write_measurement(bytes, Measurement::Byte, &mut file)?;

    // 'outer: for measurement in MEASUREMENTS.iter().rev() {
    // loop {
    // let bytes_size = Bytes::from(*measurement);
    //
    // if size < bytes_size.0 {
    // println!("sadge");
    // continue 'outer;
    // }
    //
    // // Yo I am confused about this too dw I am working on figuring it out
    // let num = size / bytes_size.0;
    // size -= num;
    //
    // let num = num / bytes_size.0;
    //
    // let bytes = BytesArray::from(*measurement);
    // for _ in 0..(num * bytes_size.0) {
    // file.write_all(&bytes.0)?;
    // }
    //
    // if size == 0 {
    // break 'outer;
    // }
    // }
    // }

    Ok(())
}
