use crate::types::Types;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tuple {
    tuple: Vec<Types>,
}

impl Tuple {
    /// Returns a [TupleBuilder]
    pub fn builder() -> TupleBuilder {
        TupleBuilder::default()
    }

    /// The number of elements in the tuple.
    pub fn len(&self) -> usize {
        self.tuple.len()
    }

    /// `true` if the tuple size is 0, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.tuple.len() == 0
    }

    /// A tuple is `concrete` if all it's tuples are not wild card [Types].
    pub fn is_concrete(&self) -> bool {
        self.tuple.iter().all(|t| t.is_concrete())
    }
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "(")?;
        write!(
            formatter,
            "{}",
            self.tuple
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        write!(formatter, ")")?;
        Ok(())
    }
}

#[derive(Default)]
pub struct TupleBuilder {
    tuple: Vec<Types>,
}

impl TupleBuilder {
    pub fn build(self) -> Tuple {
        let TupleBuilder { tuple } = self;

        Tuple { tuple }
    }

    pub fn any(mut self) -> Self {
        self.tuple.push(Types::Any);
        self
    }

    pub fn any_integer(mut self) -> Self {
        self.tuple.push(Types::AnyInteger);
        self
    }

    pub fn integer(mut self, integer: i64) -> Self {
        self.tuple.push(Types::Integer(integer));
        self
    }

    pub fn any_float(mut self) -> Self {
        self.tuple.push(Types::AnyFloat);
        self
    }

    pub fn float(mut self, float: f64) -> Self {
        self.tuple.push(Types::Float(float));
        self
    }

    pub fn any_boolean(mut self) -> Self {
        self.tuple.push(Types::AnyBoolean);
        self
    }

    pub fn boolean(mut self, boolean: bool) -> Self {
        self.tuple.push(Types::Boolean(boolean));
        self
    }

    pub fn any_string(mut self) -> Self {
        self.tuple.push(Types::AnyString);
        self
    }

    pub fn string(mut self, string: &str) -> Self {
        self.tuple.push(Types::String(String::from(string)));
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
        &self.tuple[index]
    }
}

#[test]
fn test_builder() {
    let tuple = Tuple::builder().integer(5).build();
    assert!(tuple.is_concrete());

    assert_eq!(1, tuple.len());

    let tuple = Tuple::builder()
        .any()
        .any_integer()
        .integer(1)
        .any_float()
        .float(2.0)
        .any_boolean()
        .boolean(true)
        .string("String")
        .any_string()
        .build();
    assert_eq!(9, tuple.len());
    assert!(!tuple.is_concrete());
}

#[test]
fn test_tuple_template() {
    let tuple = Tuple::builder().integer(5).integer(2).build();

    let tuple_template = Tuple::builder().integer(5).any_integer().build();

    assert_eq!(tuple_template, tuple);

    let tuple_template = Tuple::builder().any_integer().any_integer().build();
    assert_eq!(tuple_template, tuple);

    let tuple_template = Tuple::builder().integer(5).any_integer().build();
    assert_eq!(tuple_template, tuple);

    let tuple_template = Tuple::builder().integer(5).any().build();
    assert_eq!(tuple_template, tuple);

    let tuple_template = Tuple::builder().integer(5).build();
    assert_ne!(tuple_template, tuple);

    let tuple_template = Tuple::builder().build();
    assert_ne!(tuple_template, tuple);
}
