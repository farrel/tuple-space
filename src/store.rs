use crate::result::Result;
use crate::tuple::Tuple;

pub trait Store: Default {
    /// The number of tuples in the store.
    fn len(&self) -> usize;

    /// Writes a tuple into the store.
    fn write(&mut self, tuple: &Tuple) -> Result<()>;

    /// Reads a tuple from the store, matching the template tuple. Does not remove the tuple from
    /// the store.
    fn read(&self, template: &Tuple) -> Result<Option<Tuple>>;

    /// Reads a tuple from the store, matching the template tuple. Removes the tuple from
    /// the store.
    fn take(&mut self, template: &Tuple) -> Result<Option<Tuple>>;
}
