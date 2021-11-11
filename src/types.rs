use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Types {
    Integer(usize),
    Float(f64),
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
}
