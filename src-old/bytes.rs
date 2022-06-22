use crate::args::Measurement;

#[derive(Debug, Copy, Clone)]
pub struct Bytes(pub u128);

#[derive(Debug, Clone)]
pub struct BytesArray(pub Vec<u8>);

impl From<Bytes> for BytesArray {
    fn from(bytes: Bytes) -> Self {
        BytesArray(vec![0u8; bytes.0 as usize])
    }
}

impl From<Measurement> for BytesArray {
    fn from(measurement: Measurement) -> Self {
        let bytes: Bytes = measurement.into();
        Self::from(bytes)
    }
}

impl From<Measurement> for Bytes {
    fn from(measurement: Measurement) -> Self {
        Self(1024_u128.pow(measurement as u32))
    }
}
