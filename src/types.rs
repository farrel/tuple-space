use serde::{Deserialize, Serialize};

const FIRST_COMPARISON: bool = true;
const SWAPPED_COMPARISON: bool = false;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Types {
    Any,
    AnyBoolean,
    Boolean(bool),
    AnyInteger,
    Integer(usize),
    AnyFloat,
    Float(f64),
    AnyString,
    String(String),
}

impl Types {
    /// Tests whehter two types satisfy each other. If first_comparison is true, then
    fn satisfy(&self, rhs: &Types, first_comparison: bool) -> bool {
        match (self, rhs) {
            (Types::Any, _) => true,
            (Types::AnyBoolean, Types::Boolean(_)) => true,
            (Types::Boolean(lhs), Types::Boolean(rhs)) => lhs == rhs,
            (Types::AnyInteger, Types::Integer(_)) => true,
            (Types::Integer(lhs), Types::Integer(rhs)) => lhs == rhs,
            (Types::AnyFloat, Types::Float(_)) => true,
            (Types::Float(lhs), Types::Float(rhs)) => lhs == rhs,
            (Types::AnyString, Types::String(_)) => true,
            (Types::String(lhs), Types::String(rhs)) => lhs == rhs,
            _ if first_comparison => rhs.satisfy(self, SWAPPED_COMPARISON),
            _ => false,
        }
    }

    pub fn is_concrete(&self) -> bool {
        match self {
            Types::Any
            | Types::AnyBoolean
            | Types::AnyInteger
            | Types::AnyFloat
            | Types::AnyString => false,
            _ => true,
        }
    }
}

impl PartialEq for Types {
    fn eq(&self, rhs: &Types) -> bool {
        self.satisfy(rhs, FIRST_COMPARISON)
    }
}

#[test]
fn test_compare() {
    let b1 = Types::Boolean(true);
    let b1_copy = Types::Boolean(true);
    let b2 = Types::Boolean(false);

    assert_eq!(b1, b1_copy);
    assert_ne!(b1, b2);

    let i1 = Types::Integer(1);
    let i1_copy = Types::Integer(1);
    let i2 = Types::Integer(2);

    assert_eq!(i1, i1_copy);
    assert_ne!(i1, i2);

    let f1 = Types::Float(1.0);
    let f1_copy = Types::Float(1.0);
    let f2 = Types::Float(2.0);

    assert_eq!(f1, f1_copy);
    assert_ne!(f1, f2);
    assert_ne!(f1, i1);

    let s1 = Types::String(String::from("S1"));
    let s1_copy = Types::String(String::from("S1"));
    let s2 = Types::String(String::from("S2"));

    assert_eq!(s1, s1_copy);
    assert_ne!(s1, s2);
    assert_ne!(s1, f1);

    assert_eq!(i1, Types::Any);
    assert_eq!(b1, Types::AnyBoolean);
    assert_eq!(i1, Types::AnyInteger);
    assert_eq!(f1, Types::AnyFloat);
    assert_eq!(s1, Types::AnyString);
}
