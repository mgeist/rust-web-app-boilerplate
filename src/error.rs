use std::error::Error as stdError;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    UnknownError,
    PasswordTooShort,
    PasswordTooLong,
    PasswordMismatch,
    InvalidCredentials,
    EmailTaken,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    pub fn message(&self) -> &str {
        match self.kind {
            ErrorKind::UnknownError => "An unknown error occurred.",
            ErrorKind::PasswordTooShort => "Password must be at least 8 characters long.",
            ErrorKind::PasswordTooLong => "Password must be at most 64 characters long.",
            ErrorKind::PasswordMismatch => "Passwords do not match.",
            ErrorKind::InvalidCredentials => "Invalid credentials. Please try again",
            ErrorKind::EmailTaken => "Email is taken, please try another.",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl stdError for Error {}
