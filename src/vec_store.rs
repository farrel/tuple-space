use crate::error::Error;
use crate::result::Result;
use crate::store::Store;
use crate::tuple::Tuple;

/// VecStore is a simple tuple store using [Vec] for internal storage.
///
/// Tuples are stored as [Option]<[Tuple]> types and replaced by [None] when taken from the [Vec]. This reduces the need to constantly shift elements in the [Vec] as tuples are taken.
///
/// When the margin of [Some] elements is less than [VecStore::compact_margin] share of the total number of elements in the [Vec], the [Vec] is compacted using [Vec::retain] and all the
/// [None] elements are removed. The default value of [VecStore::compact_margin] is [DEFAULT_COMPACT_MARGIN].
#[derive(Clone)]
pub struct VecStore {
    inner: Vec<Option<Tuple>>,
    tuple_count: usize,
    compact_margin: f64,
}

pub const DEFAULT_COMPACT_MARGIN: f64 = 0.9;

impl VecStore {
    /// The number of [Some] enum values containing [Tuple] structs currently in the [Vec].
    pub fn tuple_count(&self) -> usize {
        self.tuple_count
    }

    /// The current compact margin threshold below which the [Vec] will be compacted. Defaults to
    /// [DEFAULT_COMPACT_MARGIN]
    pub fn compact_margin(&self) -> f64 {
        self.compact_margin
    }

    fn compact(&mut self) {
        let current_compact_margin = self.tuple_count as f64 / self.inner.len() as f64;
        if current_compact_margin < self.compact_margin {
            self.inner.retain(|t| t.is_some())
        }
    }

    /// Returns a [VecStoreBuilder] so that the initial parameters of the [VecStore] can be
    /// modified.
    pub fn builder() -> VecStoreBuilder {
        VecStoreBuilder::new()
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
}

impl Default for VecStore {
    fn default() -> Self {
        Self {
            inner: Vec::new(),
            tuple_count: 0,
            compact_margin: DEFAULT_COMPACT_MARGIN,
        }
    }
}

impl Store for VecStore {
    fn size(&self) -> Result<usize> {
        Ok(self.tuple_count)
    }

    fn read(&self, template: &Tuple) -> Result<Option<Tuple>> {
        match self.index_of(template) {
            Some(index) => Ok(self.inner[index].clone()),
            None => Ok(None),
        }
    }

    fn write(&mut self, tuple: &Tuple) -> Result<()> {
        if !tuple.is_concrete() {
            return Err(Error::NonConcreteTuple(tuple.clone()));
        }

        self.compact();
        self.inner.push(Some(tuple.clone()));
        self.tuple_count += 1;
        Ok(())
    }

    fn take(&mut self, template: &Tuple) -> Result<Option<Tuple>> {
        match self.index_of(template) {
            Some(ref index) => {
                let tuple = self.inner[*index].take();
                self.tuple_count -= 1;
                return Ok(tuple);
            }
            None => Ok(None),
        }
    }
}

pub struct VecStoreBuilder {
    compact_margin: f64,
}

impl VecStoreBuilder {
    pub fn new() -> Self {
        Self {
            compact_margin: DEFAULT_COMPACT_MARGIN,
        }
    }

    pub fn compact_margin(mut self, compact_margin: f64) -> Self {
        self.compact_margin = compact_margin;
        self
    }

    pub fn build(self) -> VecStore {
        let VecStoreBuilder { compact_margin } = self;
        VecStore {
            compact_margin,
            ..Default::default()
        }
    }
}

#[test]
fn test_store() -> Result<()> {
    use crate::error::Error;

    let mut tuple_store = VecStore::builder().compact_margin(0.85).build();

    match tuple_store.write(&Tuple::builder().any_integer().build()) {
        Err(Error::NonConcreteTuple(_)) => (),
        Ok(_) => panic!("Ok not expected"),
        Err(error) => panic!("Unexpected error {:?}", error),
    }

    tuple_store.write(&Tuple::builder().integer(5).build());
    tuple_store.write(&Tuple::builder().integer(2).build());

    assert_eq!(2, tuple_store.size()?);
    assert_eq!(2, tuple_store.tuple_count());

    match tuple_store.read(&Tuple::builder().integer(2).build())? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(2, tuple_store.size()?);
    assert_eq!(2, tuple_store.tuple_count());

    match tuple_store.take(&Tuple::builder().integer(5).build())? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(1, tuple_store.size()?);
    assert_eq!(1, tuple_store.tuple_count());

    match tuple_store.take(&Tuple::builder().any_integer().build())? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(0, tuple_store.size()?);
    assert_eq!(0, tuple_store.tuple_count());

    match tuple_store.take(&Tuple::builder().any_integer().build())? {
        Some(_tuple) => panic!("Tuple found"),
        None => (),
    }
    assert_eq!(0, tuple_store.size()?);
    assert_eq!(0, tuple_store.tuple_count());

    Ok(())
}
