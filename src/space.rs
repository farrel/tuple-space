use crate::result::Result;
use crate::store::Store;
use crate::template::TupleTemplate;
use crate::tuple::Tuple;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default, Clone)]
pub struct Space {
    store: Arc<Mutex<Store>>,
}

impl Space {
    pub fn len(&self) -> Result<usize> {
        Ok(self.store.lock()?.len())
    }

    pub fn read(&self, template: TupleTemplate) -> Result<Option<Tuple>> {
        Ok(self.store.lock()?.read(template))
    }

    pub fn write(&mut self, tuple: Tuple) -> Result<()> {
        Ok(self.store.lock()?.write(tuple))
    }

    pub fn take(&mut self, template: TupleTemplate) -> Result<Option<Tuple>> {
        Ok(self.store.lock()?.take(template))
    }
}

#[test]
fn test_space() -> Result<()> {
    let mut tuple_space = Space::default();

    tuple_space.write(Tuple::builder().add_integer(5).build());
    tuple_space.write(Tuple::builder().add_integer(2).build());

    assert_eq!(2, tuple_space.len()?);

    let test_thread = thread::spawn(|| {
        match tuple_space.read(TupleTemplate::builder().add_integer(2).build())? {
            Some(tuple) => (),
            None => panic!("No tuple found"),
        }
    });

    assert_eq!(2, tuple_space.len()?);

    match tuple_space.take(TupleTemplate::builder().add_integer(5).build())? {
        Some(tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(1, tuple_space.len()?);

    match tuple_space.take(TupleTemplate::builder().add_integer_type().build())? {
        Some(tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(0, tuple_space.len()?);

    match tuple_space.take(TupleTemplate::builder().add_integer_type().build())? {
        Some(tuple) => panic!("Tuple found"),
        None => (),
    }

    test_thread.join();
}
