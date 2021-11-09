use crate::types::Types;

#[derive(PartialEq, Debug)]
pub struct Tuple {
    inner: Vec<Types>,
}

impl Tuple {
    pub fn builder() -> TupleBuilder {
        TupleBuilder::new()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct TupleBuilder {
    inner: Vec<Types>,
}

impl TupleBuilder {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn build(self) -> Tuple {
        let TupleBuilder { inner } = self;

        Tuple { inner }
    }

    pub fn add_integer(mut self, integer: usize) -> Self {
        self.inner.push(Types::Integer(integer));
        self
    }
}

#[test]
fn test_builder() {
    let tuple = Tuple::builder().add_integer(5).build();

    assert_eq!(1, tuple.len());
}
