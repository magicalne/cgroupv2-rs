use thiserror::Error;
#[derive(Error, Debug)]
pub enum CGroupError {
    #[error("cannot open file")]
    FileSystemFailure(#[from] std::io::Error),
    #[error("unknown controller")]
    UnknownField(String),
    //TODO

    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, CGroupError>;