macro_rules! square {
    ($num:expr) => {
        $num * $num
    };
}

const BYTE: u64 = 0x01;
const KILOBYTE: u64 = 1024;
const MEGABYTE: u64 = square!(KILOBYTE);

fn main() {}
