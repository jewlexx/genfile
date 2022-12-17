use std::{fs::File, io::Write};

mod args;
mod bytes;
mod error;

use args::{GenArgs, Measurement};
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

    let measurement = args.measurement;

    let size: u128 = args.size * Bytes::from(measurement).0;

    let cwd = std::env::current_dir()?;
    let path = cwd.join(args.path.unwrap_or_else(|| "file.txt".into()));

    let gigabytes = size / Bytes::from(Measurement::Gigabyte).0;
    let megabytes = size / Bytes::from(Measurement::Megabyte).0 - gigabytes;
    let kilobytes = size / Bytes::from(Measurement::Kilobyte).0 - gigabytes - megabytes;
    let bytes = size / Bytes::from(Measurement::Byte).0 - gigabytes - megabytes - kilobytes;

    println!(
        "GB: {}\nMB: {}\nKB: {}\nB: {}",
        gigabytes, megabytes, kilobytes, bytes
    );

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
