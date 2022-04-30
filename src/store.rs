use crate::query_tuple::QueryTuple;
use crate::result::Result;
use crate::tuple::Tuple;

pub trait Store: Default {
    /// The number of tuples in the store.
    fn size(&self) -> Result<usize>;

    /// Writes a tuple into the store.
    fn write(&mut self, tuple: &Tuple) -> Result<()>;

    /// Reads a tuple from the store, matching the query tuple. Does not remove the tuple from
    /// the store.
    fn read(&self, query_tuple: &QueryTuple) -> Result<Option<Tuple>>;

    /// Reads a tuple from the store, matching the query tuple. Removes the tuple from
    /// the store.
    fn take(&mut self, query_tuple: &QueryTuple) -> Result<Option<Tuple>>;
}
