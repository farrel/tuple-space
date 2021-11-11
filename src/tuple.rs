use crate::types::Types;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
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

impl std::ops::Index<usize> for Tuple {
    type Output = Types;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

#[test]
fn test_builder() {
    let tuple = Tuple::builder().add_integer(5).build();

    assert_eq!(1, tuple.len());

    let tuple = Tuple::builder()
        .add_integer(1)
        .add_integer(2)
        .add_integer(3)
        .build();
    assert_eq!(3, tuple.len());
}
