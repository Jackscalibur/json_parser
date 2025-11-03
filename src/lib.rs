use std::collections::HashMap;

mod tokenize;

pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

