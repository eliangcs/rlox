use std::fmt;

#[derive(Copy, Clone)]
pub struct BoolValue {
    pub boolean: bool,
}

#[derive(Copy, Clone)]
pub struct NumberValue {
    pub number: f64,
}

#[derive(Copy, Clone)]
pub enum Value {
    Nil,
    Bool(BoolValue),
    Number(NumberValue),
}

impl Value {
    pub fn nil() -> Self {
        Value::Nil
    }

    pub fn boolean(value: bool) -> Self {
        Value::Bool(BoolValue { boolean: value })
    }

    pub fn number(value: f64) -> Self {
        Value::Number(NumberValue { number: value })
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(value) => write!(f, "{}", value.boolean),
            Value::Number(value) => write!(f, "{}", value.number),
        }
    }
}
