use crate::object::{Obj, StringObj};
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    Obj(Obj),
}

impl Value {
    #[inline]
    pub fn string(ptr: *const u8, len: usize) -> Value {
        Value::Obj(Obj::StringObj(StringObj { ptr: ptr, len: len }))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Number(value) => write!(f, "{}", value),
            Value::Obj(value) => match value {
                Obj::StringObj(obj) => unsafe { write!(f, "{}", obj.as_str()) },
            },
        }
    }
}
