use crate::result::Result;
use crate::store::Store;
use crate::tuple::Tuple;
use log::debug;

/// VecStore is a simple tuple store using [Vec] for internal storage.
///
/// To minimise constant shifting of the elements when a tuple is removed tuples are stored as
/// [Option]<[Tuple]> types, and are merely replaced by [None]. When the number of [None]
/// elements rises to more than [VecStore::compact_margin] share of the total number of elements, the store is compacted using [Vec::retain] and all the
/// [None] elements are removed. The default value of [VecStore::compact_margin] is 0.9.
#[derive(Clone)]
pub struct VecStore {
    inner: Vec<Option<Tuple>>,
    tuple_count: usize,
    compact_margin: f64,
}

impl VecStore {
    fn tuple_count(&self) -> usize {
        self.tuple_count
    }

    fn index_of(&self, template: &Tuple) -> Option<usize> {
        let mut index = 0;
        let inner_len = self.inner.len();
        while index < inner_len {
            match self.inner[index] {
                Some(ref tuple) => {
                    if template == tuple {
                        return Some(index);
                    }
                }
                None => (),
            }
            index += 1;
        }
        None
    }

    fn compact_margin(&self) -> f64 {
        self.compact_margin
    }

    fn compact(&mut self) {
        let current_compact_margin = (self.tuple_count as f64 / self.inner.len() as f64);
        debug!("Compact margin: {}", current_compact_margin);
        if current_compact_margin < self.compact_margin {
            self.inner.retain(|t| t.is_some())
        }
    }
}

impl Default for VecStore {
    fn default() -> Self {
        Self {
            inner: Vec::new(),
            tuple_count: 0,
            compact_margin: 0.9,
        }
    }
}

impl Store for VecStore {
    fn len(&self) -> Result<usize> {
        Ok(self.tuple_count)
    }

    fn read(&self, template: &Tuple) -> Result<Option<Tuple>> {
        match self.index_of(template) {
            Some(index) => Ok(self.inner[index].clone()),
            None => Ok(None),
        }
    }

    fn write(&mut self, tuple: &Tuple) -> Result<()> {
        self.compact();
        self.inner.push(Some(tuple.clone()));
        self.tuple_count += 1;
        Ok(())
    }

    fn take(&mut self, template: &Tuple) -> Result<Option<Tuple>> {
        match self.index_of(template) {
            Some(ref index) => {
                self.tuple_count -= 1;
                return Ok(self.inner[*index].take());
            }
            None => Ok(None),
        }
    }
}

#[test]
fn test_store() -> Result<()> {
    let mut tuple_store = VecStore::default();

    tuple_store.write(&Tuple::builder().add_integer(5).build());
    tuple_store.write(&Tuple::builder().add_integer(2).build());

    assert_eq!(2, tuple_store.len()?);
    assert_eq!(2, tuple_store.tuple_count());

    match tuple_store.read(&Tuple::builder().add_integer(2).build())? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(2, tuple_store.len()?);
    assert_eq!(2, tuple_store.tuple_count());

    match tuple_store.take(&Tuple::builder().add_integer(5).build())? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(1, tuple_store.len()?);
    assert_eq!(1, tuple_store.tuple_count());

    match tuple_store.take(&Tuple::builder().add_integer_type().build())? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(0, tuple_store.len()?);
    assert_eq!(0, tuple_store.tuple_count());

    match tuple_store.take(&Tuple::builder().add_integer_type().build())? {
        Some(_tuple) => panic!("Tuple found"),
        None => (),
    }
    assert_eq!(0, tuple_store.len()?);
    assert_eq!(0, tuple_store.tuple_count());

    Ok(())
}
