use std::io;
use std::result;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BitnessError {
    #[error("I/O error")]
    IoError(#[from] io::Error),
}

pub type BitnessResult<T> = result::Result<T, BitnessError>;
