use crate::result::Result;
use crate::store::Store;
use crate::tuple::Tuple;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct Space {
    store: Arc<Mutex<Store>>,
}

impl Space {
    pub fn len(&self) -> Result<usize> {
        Ok(self.store.lock()?.len())
    }

    pub fn read(&self, template: &Tuple) -> Result<Option<Tuple>> {
        Ok(self.store.lock()?.read(template))
    }

    pub fn write(&mut self, tuple: Tuple) -> Result<()> {
        Ok(self.store.lock()?.write(tuple))
    }

    pub fn take(&mut self, template: &Tuple) -> Result<Option<Tuple>> {
        Ok(self.store.lock()?.take(template))
    }
}

#[test]
fn test_space() -> Result<()> {
    use std::thread;
    let mut tuple_space = Space::default();

    tuple_space.write(Tuple::builder().add_integer(5).build());
    tuple_space.write(Tuple::builder().add_integer(2).build());

    assert_eq!(2, tuple_space.len()?);

    let mut thread_tuple_space = tuple_space.clone();
    let test_thread = thread::spawn(move || {
        match thread_tuple_space.read(&Tuple::builder().add_integer(2).build()) {
            Ok(Some(tuple)) => (),
            _ => panic!("No tuple found"),
        }
    });

    assert_eq!(2, tuple_space.len()?);

    let exact_template = Tuple::builder().add_integer(5).build();
    let wildcard_template = Tuple::builder().add_integer_type().build();

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
