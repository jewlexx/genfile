use std::{fs::File, io::Write};

mod args;
mod bytes;
mod error;

use args::GenArgs;
use bytes::{Bytes, BytesArray};

fn main() -> anyhow::Result<()> {
    let args: GenArgs = clap::Parser::parse();

    let measurement = args.measurement;

    let size: u128 = args.size * Bytes::from(measurement).0;

    let cwd = std::env::current_dir()?;
    let path = cwd.join(args.path.unwrap_or_else(|| "file.bin".into()));

    let mut file = File::create(path)?;

    let array = BytesArray::from(measurement);

    for _ in 0..size {
        file.write_all(&array.0)?;
    }

    Ok(())
}
