use crate::store::Store;
use crate::tuple::Tuple;
use serde::{Deserialize, Serialize};

/// Error type
#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    MutexPoisonError,
    NonConcreteTuple(Tuple),
}

impl<S> From<std::sync::PoisonError<std::sync::MutexGuard<'_, S>>> for Error
where
    S: Store,
{
    fn from(_error: std::sync::PoisonError<std::sync::MutexGuard<'_, S>>) -> Error {
        Error::MutexPoisonError
    }
}
