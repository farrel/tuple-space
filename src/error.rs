use crate::store::Store;
use crate::tuple::Tuple;

#[derive(Debug)]
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
