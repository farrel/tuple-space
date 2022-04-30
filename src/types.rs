use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Types {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl Types {
    /// Tests whehter two types satisfy each other. If first_comparison is true, if there is
    /// no match between the self and other them call `other.satisfy(self, false)`.
    fn satisfy(&self, other: &Types) -> bool {
        match (self, other) {
            (Types::Boolean(lhs), Types::Boolean(rhs)) => lhs == rhs,
            (Types::Integer(lhs), Types::Integer(rhs)) => lhs == rhs,
            (Types::Float(lhs), Types::Float(rhs)) => lhs == rhs,
            (Types::String(lhs), Types::String(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl std::fmt::Display for Types {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Types::Boolean(boolean) => write!(formatter, "{}", boolean)?,
            Types::Integer(integer) => write!(formatter, "{}", integer)?,
            Types::Float(float) => write!(formatter, "{}", float)?,
            Types::String(string) => write!(formatter, "\"{}\"", string)?,
        };
        Ok(())
    }
}

/// Implements matching allowing wildcard Types to match value Types, as well as value types to
/// match each other.
impl PartialEq for Types {
    fn eq(&self, other: &Types) -> bool {
        self.satisfy(other)
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
}
