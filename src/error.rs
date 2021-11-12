use crate::store::Store;

#[derive(Debug)]
pub enum Error {
    MutexPoisonError,
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, Store>>> for Error {
    fn from(_error: std::sync::PoisonError<std::sync::MutexGuard<'_, Store>>) -> Error {
        Error::MutexPoisonError
    }
}
