use crate::query_tuple::QueryTuple;
use crate::result::Result;
use crate::store::Store;
use crate::tuple::Tuple;
use std::sync::{Arc, Mutex};

/// A [MutexStore] is a thread safe Tuple store wrapper that uses another tuple store as internal
/// storage.
///
/// The internal tuple store is encased in an [Arc]<[Mutex]<T>> making the [MutexStore] thread
/// safe across clones.
/// ```rust
/// use tuple_space::mutex_store::MutexStore;
/// use tuple_space::tuple::Tuple;
/// use tuple_space::vec_store::VecStore;
/// use tuple_space::store::Store;
///
/// fn main() -> tuple_space::result::Result<()>{
///   let mut store = MutexStore::<VecStore>::default();
///   let mut store_clone = store.clone();
///   let tuple = Tuple::builder().integer(1).build();
///
///   store.write(&tuple);
///   store_clone.write(&tuple);
///   println!("Tuples stored: {}", store.size()?);      // -> 2
///   println!("Tuples stored: {}", store_clone.size()?); // -> 2
///   Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct MutexStore<S: Store> {
    store: Arc<Mutex<S>>,
}

impl<S: Store> MutexStore<S> {}

impl<S: Store> Default for MutexStore<S> {
    fn default() -> MutexStore<S> {
        MutexStore {
            store: Arc::new(Mutex::new(S::default())),
        }
    }
}

impl<S: Store> Store for MutexStore<S> {
    fn size(&self) -> Result<usize> {
        self.store.lock()?.size()
    }

    fn write(&mut self, tuple: &Tuple) -> Result<()> {
        Ok(self.store.lock()?.write(tuple)?)
    }

    fn read(&self, query_tuple: &QueryTuple) -> Result<Option<Tuple>> {
        self.store.lock()?.read(query_tuple)
    }

    fn take(&mut self, query_tuple: &QueryTuple) -> Result<Option<Tuple>> {
        match self.store.lock()?.take(query_tuple)? {
            Some(tuple) => Ok(Some(tuple)),
            None => Ok(None),
        }
    }
}

#[test]
fn test_mutex_store() -> Result<()> {
    use crate::store::Store;
    use crate::tuple::Tuple;
    use crate::vec_store::VecStore;
    use std::thread;

    let mut tuple_space = MutexStore::<VecStore>::default();

    tuple_space.write(&Tuple::builder().integer(5).build())?;
    tuple_space.write(&Tuple::builder().integer(2).build())?;

    assert_eq!(2, tuple_space.size()?);

    let thread_tuple_space = tuple_space.clone();
    let test_thread = thread::spawn(move || {
        match thread_tuple_space.read(&QueryTuple::builder().integer(2).build()) {
            Ok(Some(_tuple)) => (),
            _ => panic!("No tuple found"),
        }
    });
    if let Err(err) = test_thread.join() {
        panic!("{:?}", err);
    }

    assert_eq!(2, tuple_space.size()?);

    let exact_query_tuple = QueryTuple::builder().integer(5).build();
    let wildcard_query_tuple = QueryTuple::builder().any_integer().build();

    match tuple_space.take(&exact_query_tuple)? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(1, tuple_space.size()?);

    match tuple_space.take(&wildcard_query_tuple)? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(0, tuple_space.size()?);

    match tuple_space.take(&wildcard_query_tuple)? {
        Some(_tuple) => panic!("Tuple found"),
        None => (),
    }

    Ok(())
}
