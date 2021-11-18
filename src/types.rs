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
            (Types::Boolean(b_1), Types::Boolean(b_2)) => b_1 == b_2,
            (Types::AnyInteger, Types::Integer(_)) => true,
            (Types::Integer(n_1), Types::Integer(n_2)) => n_1 == n_2,
            (Types::AnyFloat, Types::Float(_)) => true,
            (Types::Float(f_1), Types::Float(f_2)) => f_1 == f_2,
            _ if first_comparison => rhs.satisfy(self, SWAPPED_COMPARISON),
            _ => false,
        }
    }

    pub fn is_concrete(&self) -> bool {
        match self {
            Types::Any | Types::AnyBoolean | Types::AnyInteger | Types::AnyFloat => false,
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

    assert_eq!(i1, Types::Any);
    assert_eq!(b1, Types::AnyBoolean);
    assert_eq!(i1, Types::AnyInteger);
    assert_eq!(f1, Types::AnyFloat);
}
