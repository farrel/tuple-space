use crate::types::Types;

#[derive(PartialEq, Debug)]
pub enum Template {
    IntegerType,
    Integer(usize),
}

impl Template {
    pub(crate) fn partial_eq_integer_type(&self, rhs: &Types) -> bool {
        if let Types::Integer(_n) = rhs {
            true
        } else {
            false
        }
    }

    pub(crate) fn partial_eq_integer(&self, rhs: &Types) -> bool {
        if let Template::Integer(template_value) = self {
            if let Types::Integer(type_value) = rhs {
                template_value == type_value
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl PartialEq<Types> for Template {
    fn eq(&self, rhs: &Types) -> bool {
        match self {
            Self::IntegerType => self.partial_eq_integer_type(rhs),
            Self::Integer(_) => self.partial_eq_integer(rhs),
        }
    }
}

#[test]
fn test_integer_type() {
    assert_eq!(Template::IntegerType, Types::Integer(1));
    assert_eq!(Template::Integer(1), Types::Integer(1));
    //assert_ne!(Template::Integer(1), Types::Integer(2));
}
