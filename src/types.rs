use serde::{Deserialize, Serialize};

const FIRST_COMPARISON: bool = false;
const SWAPPED_COMPARISON: bool = true;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Types {
    Any,
    AnyInteger,
    Integer(usize),
    AnyFloat,
    Float(f64),
}

impl Types {
    fn satisfy(&self, rhs: &Types, comparison: bool) -> bool {
        match (self, rhs) {
            (Types::Any, _) => true,
            (Types::AnyInteger, Types::Integer(_)) => true,
            (Types::Integer(n_1), Types::Integer(n_2)) => n_1 == n_2,
            (Types::AnyFloat, Types::Float(_)) => true,
            (Types::Float(f_1), Types::Float(f_2)) => f_1 == f_2,
            _ if comparison == FIRST_COMPARISON => rhs.satisfy(self, SWAPPED_COMPARISON),
            _ => false,
        }
    }

    pub fn is_concrete(&self) -> bool {
        match self {
            Types::Any | Types::AnyInteger | Types::AnyFloat => false,
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
    assert_eq!(i1, Types::AnyInteger);
}
