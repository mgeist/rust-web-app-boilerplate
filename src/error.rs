#[derive(Debug)]
pub enum Error {
    UnknownError,
    PasswordTooShort,
    PasswordTooLong,
    PasswordMismatch,
}
