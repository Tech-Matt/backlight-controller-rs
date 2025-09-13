use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::num;

#[derive(Debug)]
pub enum Error {
    MissingArg,
    ParseBrightness(num::ParseIntError),
    Dbus(dbus::Error)
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingArg => write!(f, 
                "Missing Brightness Argument. Usage: bright-rs <u32>"),
            Error::ParseBrightness(e) => write!(f,
                "Invalid Brightness Value {e}"),
            Error::Dbus(e) => write!(f, "Dbus error: {e}"),
        }       
    }   
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::MissingArg => None,
            Error::ParseBrightness(e) => Some(e),
            Error::Dbus(e) => Some(e),
        }
    }
}

impl From<dbus::Error> for Error {
    fn from(e: dbus::Error) -> Self {
        Error::Dbus(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseBrightness(e)
    }
}