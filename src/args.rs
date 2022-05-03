use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "A simple demo file generator")]
pub struct Args {
    #[argh(positional)]
    size: usize,

    #[argh(option, short = 'm', description = "the data measurement to write")]
    measurement: Option<char>,

    #[argh(option, short = 'f', description = "the file to write to")]
    filename: Option<String>,
}
