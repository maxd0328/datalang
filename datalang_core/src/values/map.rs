use std::collections::HashMap;
use super::{Value, ValueFeatures};
use super::union::Union;

#[derive(Debug, Clone)]
pub struct Map {
   pub entries: HashMap<String, Value>,
   valueschema: Box<Value>
}

impl Map {
   pub fn new(entries: HashMap<String, Value>) -> Self {
      let valueschema = Box::new(Value::Union(Union(entries.values().map(|v| v.clone()).collect())));
      Self { entries, valueschema }
   }
}

impl ValueFeatures for Map {
   fn select_key(&self, key: &str) -> Option<&Value> {
      match key {
         "valueschema" => Some(&self.valueschema),
         _ => self.entries.get(key)
      }
   }
}
