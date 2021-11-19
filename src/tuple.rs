use crate::types::Types;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tuple {
    inner: Vec<Types>,
}

impl Tuple {
    /// Returns a [TupleBuilder]
    pub fn builder() -> TupleBuilder {
        TupleBuilder::new()
    }

    /// The number of elements in the tuple.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// A tuple is `concrete` if all it's tuples are not wild card [Types].
    pub fn is_concrete(&self) -> bool {
        self.inner.iter().all(|t| t.is_concrete())
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

    pub fn add_any(mut self) -> Self {
        self.inner.push(Types::Any);
        self
    }

    pub fn add_integer_type(mut self) -> Self {
        self.inner.push(Types::AnyInteger);
        self
    }

    pub fn add_integer(mut self, integer: i64) -> Self {
        self.inner.push(Types::Integer(integer));
        self
    }

    pub fn add_float_type(mut self) -> Self {
        self.inner.push(Types::AnyFloat);
        self
    }

    pub fn add_float(mut self, float: f64) -> Self {
        self.inner.push(Types::Float(float));
        self
    }

    pub fn add_boolean_type(mut self) -> Self {
        self.inner.push(Types::AnyBoolean);
        self
    }

    pub fn add_boolean(mut self, boolean: bool) -> Self {
        self.inner.push(Types::Boolean(boolean));
        self
    }

    pub fn add_string_type(mut self) -> Self {
        self.inner.push(Types::AnyString);
        self
    }

    pub fn add_string(mut self, string: &str) -> Self {
        self.inner.push(Types::String(String::from(string)));
        self
    }
}

impl PartialEq for Tuple {
    fn eq(&self, rhs: &Tuple) -> bool {
        if self.len() != rhs.len() {
            return false;
        }

        for i in 0..self.len() {
            if self[i] != rhs[i] {
                return false;
            }
        }
        true
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
    assert!(tuple.is_concrete());

    assert_eq!(1, tuple.len());

    let tuple = Tuple::builder()
        .add_any()
        .add_integer_type()
        .add_integer(1)
        .add_float_type()
        .add_float(2.0)
        .add_boolean_type()
        .add_boolean(true)
        .add_string("String")
        .add_string_type()
        .build();
    assert_eq!(9, tuple.len());
    assert!(!tuple.is_concrete());
}

#[test]
fn test_tuple_template() {
    let tuple = Tuple::builder().add_integer(5).add_integer(2).build();

    let tuple_template = Tuple::builder().add_integer(5).add_integer_type().build();

    assert_eq!(tuple_template, tuple);

    let tuple_template = Tuple::builder()
        .add_integer_type()
        .add_integer_type()
        .build();
    assert_eq!(tuple_template, tuple);

    let tuple_template = Tuple::builder().add_integer(5).add_integer_type().build();
    assert_eq!(tuple_template, tuple);

    let tuple_template = Tuple::builder().add_integer(5).add_any().build();
    assert_eq!(tuple_template, tuple);

    let tuple_template = Tuple::builder().add_integer(5).build();
    assert_ne!(tuple_template, tuple);

    let tuple_template = Tuple::builder().build();
    assert_ne!(tuple_template, tuple);
}
