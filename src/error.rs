use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BitnessError {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[cfg(target_os = "freebsd")]
    #[error("Sysctl error")]
    Sysctl(String),
}
