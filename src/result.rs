use std::io;
use std::result;

#[derive(Debug, Fail)]
pub enum BitnessError {
    #[fail(display = "I/O error: {}", description)]
    IoError {
        description: String,
    },

    #[fail(display = "{}", description)]
    Error {
        description: String,
    },
}

impl From<String> for BitnessError {
    fn from(err: String) -> Self {
        BitnessError::Error {
            description: err,
        }
    }
}

impl From<io::Error> for BitnessError {
    fn from(err: io::Error) -> Self {
        BitnessError::IoError {
            description: err.to_string(),
        }
    }
}

pub type BitnessResult<T> = result::Result<T, BitnessError>;
