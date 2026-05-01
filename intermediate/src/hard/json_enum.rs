/*
  Problem 65: Data Processing — JSON-like Enum

  Define an enum Value representing a simplified JSON value:
  Null, Bool(bool), Number(f64), String(String), Array(Vec<Value>),
  and Object(HashMap<String, Value>). Implement a method fn to_json_string(&self) -> String
  that produces a JSON-formatted string.

  Run the tests for this problem with:
    cargo test --test json_enum_test
*/

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn to_json_string(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),

            Value::String(s) => format!("\"{}\"", s),

            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_json_string()).collect();
                format!("[{}]", elements.join(", "))
            }

            Value::Object(obj) => {
                let mut pairs: Vec<String> = obj.iter().map(|(k,v)| format!("\"{}\": {}", k, v.to_json_string())).collect();

                pairs.sort();
                
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }
}
