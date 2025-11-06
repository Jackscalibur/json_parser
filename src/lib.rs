mod tokenize;

use std::collections::HashMap;
use tokenize::tokenize;


pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}
