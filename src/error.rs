use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse unit")]
    UnitParseError(String),
    #[error("Failed to perform calculation")]
    CalculationError(fasteval::Error),
}
