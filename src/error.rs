use dicom::object::ReadError;
use dicom::pixeldata::Error;
use std::{error, fmt, io, result};

#[derive(Debug)]
pub enum CliError {
    Read(ReadError),
    PixelData(Error),
    File(io::Error),
}

pub type Result<T> = result::Result<T, CliError>;

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::Read(ref err) => write!(f, "{err}"),
            CliError::PixelData(ref err) => write!(f, "{err}"),
            CliError::File(ref err) => write!(f, "{err}"),
        }
    }
}

impl error::Error for CliError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            CliError::Read(ref err) => Some(err),
            CliError::PixelData(ref err) => Some(err),
            CliError::File(ref err) => Some(err),
        }
    }
}

impl From<ReadError> for CliError {
    fn from(err: ReadError) -> CliError {
        CliError::Read(err)
    }
}

impl From<Error> for CliError {
    fn from(err: Error) -> CliError {
        CliError::PixelData(err)
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::File(err)
    }
}
