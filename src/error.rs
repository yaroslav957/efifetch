use core::{error, fmt, result};
use uefi::Status;

#[non_exhaustive]
pub enum Error {
    Uefi(Status),
    UefiData(Status, &'static str),
    Fmt(fmt::Error),
}

impl Error {
    pub fn status(&self) -> Status {
        match self {
            Error::Uefi(status) => *status,
            Error::UefiData(status, _) => *status,
            _ => Status::ABORTED, // Generic err
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Uefi(status) => {
                write!(f, "[efifetch] UEFI error: {}", status)
            }
            Error::UefiData(status, data) => {
                write!(f, "[efifetch] UEFI error {}: {}", status, data)
            }
            Error::Fmt(e) => write!(f, "[efifetch] Stdout/fmt error: {}", e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Uefi(status) => write!(f, "UEFI error: {}", status),
            Error::UefiData(status, data) => {
                write!(f, "UEFI error {}: {}", status, data)
            }
            Error::Fmt(e) => write!(f, "Stdout/fmt error: {}", e),
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

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::Fmt(e)
    }
}

pub type Result<T> = result::Result<T, Error>;
