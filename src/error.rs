use core::{error, fmt, result};
use heapless::{CapacityError, string::FromUtf16Error};
use uefi::Status;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Fmt(fmt::Error),
    Capacity(CapacityError),
    Utf16(FromUtf16Error),
    Uefi(Status),
    UefiData(Status, &'static str),
}

impl Error {
    pub fn status(&self) -> Status {
        match self {
            Error::Uefi(status) => *status,
            Error::UefiData(status, _) => *status,
            _ => Status::ABORTED,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Fmt(e) => {
                write!(f, "Stdout UEFI proto or fmt error: {e}")
            }
            Error::Capacity(e) => {
                write!(f, "Capactiy error of the existing buf: {e}")
            }
            Error::Utf16(e) => write!(f, "Invalid UTF16 sequence: {e}"),
            Error::Uefi(status) => write!(f, "UEFI error: {status}"),
            Error::UefiData(status, data) => {
                write!(f, "UEFI error: {status}, with data: {data}")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Capacity(e) => Some(e),
            Error::Fmt(e) => Some(e),
            _ => None,
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::Fmt(e)
    }
}

impl From<CapacityError> for Error {
    fn from(e: CapacityError) -> Self {
        Error::Capacity(e)
    }
}

impl From<FromUtf16Error> for Error {
    fn from(e: FromUtf16Error) -> Self {
        Error::Utf16(e)
    }
}

impl From<uefi::Error> for Error {
    fn from(e: uefi::Error) -> Self {
        Error::Uefi(e.status())
    }
}

impl From<uefi::Error<&'static str>> for Error {
    fn from(e: uefi::Error<&'static str>) -> Self {
        Error::UefiData(e.status(), e.data())
    }
}

pub type Result<T> = result::Result<T, Error>;
