use crate::template::TupleTemplate;
use crate::tuple::Tuple;

#[derive(Default)]
pub(crate) struct Store {
    inner: Vec<Tuple>,
}

impl Store {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    fn index(&self, template: &TupleTemplate) -> Option<usize> {
        let mut index = 0;
        while index < self.len() {
            if *template == self.inner[index] {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    pub fn read(&self, template: &TupleTemplate) -> Option<Tuple> {
        match self.index(template) {
            Some(index) => Some(self.inner[index].clone()),
            None => None,
        }
    }

    pub fn write(&mut self, tuple: Tuple) {
        self.inner.push(tuple)
    }

    pub fn take(&mut self, template: &TupleTemplate) -> Option<Tuple> {
        match self.index(template) {
            Some(index) => Some(self.inner.swap_remove(index)),
            None => None,
        }
    }
}

#[test]
fn test_store() {
    let mut tuple_store = Store::default();

    tuple_store.write(Tuple::builder().add_integer(5).build());
    tuple_store.write(Tuple::builder().add_integer(2).build());

    assert_eq!(2, tuple_store.len());

    match tuple_store.read(TupleTemplate::builder().add_integer(2).build()) {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(2, tuple_store.len());

    match tuple_store.take(TupleTemplate::builder().add_integer(5).build()) {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(1, tuple_store.len());

    match tuple_store.take(TupleTemplate::builder().add_integer_type().build()) {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(0, tuple_store.len());

    match tuple_store.take(TupleTemplate::builder().add_integer_type().build()) {
        Some(_tuple) => panic!("Tuple found"),
        None => (),
    }
}
