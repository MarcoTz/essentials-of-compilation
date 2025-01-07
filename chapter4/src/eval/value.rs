use crate::{
    errors::Error,
    syntax::{types::Type, Exp},
};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

impl Value {
    pub fn as_int(self) -> Result<i64, Error> {
        if let Value::Int(i) = self {
            Ok(i)
        } else {
            Err(Error::BadValue {
                found: self,
                expected: Type::Int,
            })
        }
    }

    pub fn as_bool(self) -> Result<bool, Error> {
        if let Value::Bool(b) = self {
            Ok(b)
        } else {
            Err(Error::BadValue {
                found: self,
                expected: Type::Bool,
            })
        }
    }
}

impl From<Value> for Exp {
    fn from(val: Value) -> Exp {
        match val {
            Value::Int(i) => Exp::Int(i),
            Value::Bool(b) => Exp::Bool(b),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(i) => f.write_str(&i.to_string()),
            Value::Bool(b) => f.write_str(&b.to_string()),
        }
    }
}
