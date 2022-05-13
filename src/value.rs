use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Number(value) => write!(f, "{}", value),
        }
    }
}
