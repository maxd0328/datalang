use std::collections::HashMap;
use super::{IndexSelectionError, KeySelectionError, Value, ValueFeatures};
use super::union::Union;
use crate::concrete::ConcreteValue;

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
   fn select_key(&self, key: &str) -> Result<&Value, KeySelectionError> {
      match key {
         "valueschema" => Ok(&self.valueschema),
         _ => self.entries.get(key).ok_or(KeySelectionError::NoSuchKey)
      }
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      self.entries.iter().all(|(_, v)| v.is_definite())
   }

   fn concretize(self) -> Option<ConcreteValue> {
      Some(ConcreteValue::Map(
         self.entries.into_iter().map(|(k, v)| v.concretize().map(|c| (k, c))).collect::<Option<_>>()?
      ))
   }
}
