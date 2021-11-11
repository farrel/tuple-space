use crate::tuple::Tuple;
use crate::types::Types;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum TypeTemplate {
    IntegerType,
    Integer(usize),
    FloatType,
    Float(f64),
    Any,
}

impl TypeTemplate {
    fn partial_eq_integer_type(&self, rhs: &Types) -> bool {
        if let Types::Integer(_) = rhs {
            true
        } else {
            false
        }
    }

    fn partial_eq_integer(&self, rhs: &Types) -> bool {
        if let TypeTemplate::Integer(template_value) = self {
            if let Types::Integer(type_value) = rhs {
                template_value == type_value
            } else {
                false
            }
        } else {
            false
        }
    }

    fn partial_eq_float_type(&self, rhs: &Types) -> bool {
        if let Types::Float(_) = rhs {
            true
        } else {
            false
        }
    }

    fn partial_eq_float(&self, rhs: &Types) -> bool {
        if let TypeTemplate::Float(template_value) = self {
            if let Types::Float(type_value) = rhs {
                template_value == type_value
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl PartialEq<Types> for TypeTemplate {
    fn eq(&self, rhs: &Types) -> bool {
        match self {
            Self::IntegerType => self.partial_eq_integer_type(rhs),
            Self::Integer(_) => self.partial_eq_integer(rhs),
            Self::FloatType => self.partial_eq_float_type(rhs),
            Self::Float(_) => self.partial_eq_float(rhs),
            Self::Any => true,
        }
    }
}

#[test]
fn test_integer_type() {
    assert_eq!(TypeTemplate::IntegerType, Types::Integer(1));
    assert_eq!(TypeTemplate::Integer(1), Types::Integer(1));
    assert_ne!(TypeTemplate::Integer(1), Types::Integer(2));

    assert_eq!(TypeTemplate::FloatType, Types::Float(1.0));
    assert_eq!(TypeTemplate::Float(1.0), Types::Float(1.0));
    assert_ne!(TypeTemplate::Float(1.0), Types::Float(2.0));
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct TupleTemplate {
    inner: Vec<TypeTemplate>,
}

impl TupleTemplate {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn builder() -> TupleTemplateBuilder {
        TupleTemplateBuilder::new()
    }
}

impl std::ops::Index<usize> for TupleTemplate {
    type Output = TypeTemplate;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl PartialEq<Tuple> for TupleTemplate {
    fn eq(&self, rhs: &Tuple) -> bool {
        if self.len() != rhs.len() {
            return false;
        }
        let mut index = 0;
        let size = self.len();

        while index < size {
            if self[index] != rhs[index] {
                return false;
            }
            index += 1;
        }
        true
    }
}

pub struct TupleTemplateBuilder {
    inner: Vec<TypeTemplate>,
}

impl TupleTemplateBuilder {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn build(self) -> TupleTemplate {
        let TupleTemplateBuilder { inner } = self;
        TupleTemplate { inner }
    }

    pub fn add_integer(mut self, integer: usize) -> Self {
        self.inner.push(TypeTemplate::Integer(integer));
        self
    }

    pub fn add_integer_type(mut self) -> Self {
        self.inner.push(TypeTemplate::IntegerType);
        self
    }

    pub fn add_any(mut self) -> Self {
        self.inner.push(TypeTemplate::Any);
        self
    }
}

#[test]
fn test_tuple_template() {
    let tuple = Tuple::builder().add_integer(5).add_integer(2).build();

    let tuple_template = TupleTemplate::builder()
        .add_integer(5)
        .add_integer(2)
        .build();

    assert_eq!(tuple_template, tuple);

    let tuple_template = TupleTemplate::builder()
        .add_integer_type()
        .add_integer_type()
        .build();
    assert_eq!(tuple_template, tuple);

    let tuple_template = TupleTemplate::builder()
        .add_integer(5)
        .add_integer_type()
        .build();
    assert_eq!(tuple_template, tuple);

    let tuple_template = TupleTemplate::builder().add_integer(5).add_any().build();
    assert_eq!(tuple_template, tuple);

    let tuple_template = TupleTemplate::builder().add_integer(5).build();
    assert_ne!(tuple_template, tuple);

    let tuple_template = TupleTemplate::builder().build();
    assert_ne!(tuple_template, tuple);
}
