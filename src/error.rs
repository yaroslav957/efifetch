use alloc::string::FromUtf16Error;
use core::{fmt, result, str::Utf8Error};
use thiserror::Error;
use uefi::Status;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Stdout UEFI proto or fmt error: {0}")]
    Fmt(#[from] fmt::Error),

    #[error("Invalid UTF-8 sequence: {0}")]
    Utf8(#[from] Utf8Error),

    #[error("Invalid UTF-16 sequence: {0}")]
    Utf16(#[from] FromUtf16Error),

    #[error("UEFI error: {0}")]
    Uefi(Status),

    #[error("UEFI error: {0}, with data: {1}")]
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
