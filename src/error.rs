use core::{error, fmt, result};
use heapless::string::FromUtf16Error;
use uefi::Status;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Uefi(Status),
    UefiData(Status, &'static str),
    Utf16(FromUtf16Error),
    Fmt(fmt::Error),
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
            Error::Uefi(status) => write!(f, "UEFI error: {status}"),
            Error::UefiData(status, data) => {
                write!(f, "UEFI error: {status}, with data: {data}")
            }
            Error::Utf16(e) => write!(f, "Invalid UTF16 sequence: {e}"),
            Error::Fmt(e) => write!(f, "Stdout/fmt error: {e}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Fmt(e) => Some(e),
            _ => None,
        }
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

impl From<FromUtf16Error> for Error {
    fn from(e: FromUtf16Error) -> Self {
        Error::Utf16(e)
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::Fmt(e)
    }
}

pub type Result<T> = result::Result<T, Error>;
