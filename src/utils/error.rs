use image::ImageError;
use std::fmt;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};

#[derive(Debug)]
pub enum Error {
    FileNotFound(String),
    DecodeProblem,
    UnknownError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        match err.kind() {
            IOErrorKind::NotFound => Error::FileNotFound(err.to_string()),
            _ => Error::UnknownError(err.to_string()),
        }
    }
}

impl From<ImageError> for Error {
    fn from(_: ImageError) -> Self {
        Error::DecodeProblem
    }
}

impl Error {
    fn to_string(&self) -> String {
        match self {
            Error::FileNotFound(file) => format!("Can't find the image. {}", file),
            _ => String::from("Unknown error"),
        }
    }
}
