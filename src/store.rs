use crate::result::Result;
use crate::tuple::Tuple;

pub trait Store: Default {
    fn len(&self) -> usize;
    fn write(&mut self, tuple: &Tuple) -> Result<()>;
    fn read(&self, tuple: &Tuple) -> Option<Tuple>;
    fn take(&mut self, tuple: &Tuple) -> Option<Tuple>;
}
