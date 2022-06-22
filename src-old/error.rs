#[derive(Debug, thiserror::Error, Clone)]
pub enum ArgError {
    #[error("Invalid Measurement: {0}. Must be kb, mb, or gb")]
    InvalidMeasurement(String),
}
