#[derive(PartialEq, Debug)]
pub enum Types {
    Integer(usize),
}

#[test]
fn test_compare() {
    let i1 = Types::Integer(1);
    let i1_copy = Types::Integer(1);
    let i2 = Types::Integer(2);

    assert_eq!(i1, i1_copy);
    assert_ne!(i1, i2);
}
