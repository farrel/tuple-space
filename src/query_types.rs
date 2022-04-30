use crate::types::Types;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QueryTypes {
    /// Wildcard matching any variant
    Any,
    /// Wildcard type matching any [Types::Boolean] variant
    AnyBoolean,
    /// Wildcard type matching any [Types::Integer] variant
    AnyInteger,
    /// Wildcard type matching any [Types::Float] variant
    AnyFloat,
    /// Wildcard type matching any [Types::String] variant
    AnyString,
    ExactBoolean(bool),
    ExactInteger(i64),
    ExactFloat(f64),
    ExactString(String),
}

impl QueryTypes {
    fn satisfy(&self, other: &Types) -> bool {
        match (self, other) {
            (QueryTypes::Any, _) => true,
            (QueryTypes::AnyBoolean, Types::Boolean(_)) => true,
            (QueryTypes::ExactBoolean(lhs), Types::Boolean(rhs)) => lhs == rhs,
            (QueryTypes::AnyInteger, Types::Integer(_)) => true,
            (QueryTypes::ExactInteger(lhs), Types::Integer(rhs)) => lhs == rhs,
            (QueryTypes::AnyFloat, Types::Float(_)) => true,
            (QueryTypes::ExactFloat(lhs), Types::Float(rhs)) => lhs == rhs,
            (QueryTypes::AnyString, Types::String(_)) => true,
            (QueryTypes::ExactString(lhs), Types::String(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl PartialEq<Types> for QueryTypes {
    fn eq(&self, other: &Types) -> bool {
        self.satisfy(other)
    }
}

#[test]
fn test_compare() {
    let boolean = Types::Boolean(true);
    let integer = Types::Integer(1);
    let float = Types::Float(1.0);
    let string = Types::String(String::from("S1"));

    assert_eq!(QueryTypes::Any, boolean);
    assert_eq!(QueryTypes::Any, integer);
    assert_eq!(QueryTypes::Any, float);
    assert_eq!(QueryTypes::Any, string);
    assert_eq!(QueryTypes::AnyBoolean, boolean);
    assert_eq!(QueryTypes::AnyInteger, integer);
    assert_eq!(QueryTypes::AnyFloat, float);
    assert_eq!(QueryTypes::AnyString, string);

    assert_eq!(QueryTypes::ExactString(String::from("S1")), string);
    assert_eq!(QueryTypes::ExactInteger(1), integer);
    assert_eq!(QueryTypes::ExactFloat(1.0), float);
    assert_eq!(QueryTypes::ExactBoolean(true), boolean);
}
