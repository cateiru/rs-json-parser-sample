use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Can not open file: {0}")]
    CannotOpenFileError(String),

    #[error("Parse error")]
    ParseError(),
}
