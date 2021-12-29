use crate::result::Result;
use crate::store::Store;
use crate::tuple::Tuple;
use std::sync::{Arc, Mutex};

/// A [Space] is a store that uses another tuple store as internal
/// storage.
///
/// The internal tuple store is encased in an [Arc]<[Mutex]<[Store]>> making the [Space] thread
/// safe across clones.
/// ```rust
/// use tuple_space::space::Space;
/// use tuple_space::tuple::Tuple;
/// use tuple_space::vec_store::VecStore;
/// use tuple_space::store::Store;
///
/// fn main() -> tuple_space::result::Result<()>{
///   let mut space = Space::<VecStore>::default();
///   let mut space_clone = space.clone();
///   let tuple = Tuple::builder().integer(1).build();
///
///   space.write(&tuple);
///   space_clone.write(&tuple);
///   println!("Tuples stored: {}", space.len()?);      // -> 2
///   println!("Tuples stored: {}", space_clone.len()?); // -> 2
///   Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct Space<S: Store> {
    store: Arc<Mutex<S>>,
}

impl<S: Store> Space<S> {}

impl<S: Store> Default for Space<S> {
    fn default() -> Space<S> {
        Space {
            store: Arc::new(Mutex::new(S::default())),
        }
    }
}

impl<S: Store> Store for Space<S> {
    fn len(&self) -> Result<usize> {
        Ok(self.store.lock()?.len()?)
    }

    fn read(&self, template: &Tuple) -> Result<Option<Tuple>> {
        self.store.lock()?.read(template)
    }

    fn write(&mut self, tuple: &Tuple) -> Result<()> {
        self.store.lock()?.write(tuple)?;
        Ok(())
    }

    fn take(&mut self, template: &Tuple) -> Result<Option<Tuple>> {
        match self.store.lock()?.take(template)? {
            Some(tuple) => Ok(Some(tuple)),
            None => Ok(None),
        }
    }
}

#[test]
fn test_space() -> Result<()> {
    use crate::store::Store;
    use crate::tuple::Tuple;
    use crate::vec_store::VecStore;
    use std::thread;

    let mut tuple_space = Space::<VecStore>::default();

    tuple_space.write(&Tuple::builder().integer(5).build());
    tuple_space.write(&Tuple::builder().integer(2).build());

    assert_eq!(2, tuple_space.len()?);

    let mut thread_tuple_space = tuple_space.clone();
    let test_thread = thread::spawn(move || {
        match thread_tuple_space.read(&Tuple::builder().integer(2).build()) {
            Ok(Some(tuple)) => (),
            _ => panic!("No tuple found"),
        }
    });

    assert_eq!(2, tuple_space.len()?);

    let exact_template = Tuple::builder().integer(5).build();
    let wildcard_template = Tuple::builder().any_integer().build();

    match tuple_space.take(&exact_template)? {
        Some(tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(1, tuple_space.len()?);

    match tuple_space.take(&wildcard_template)? {
        Some(tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(0, tuple_space.len()?);

    match tuple_space.take(&wildcard_template)? {
        Some(tuple) => panic!("Tuple found"),
        None => (),
    }

    test_thread.join();
    Ok(())
}
