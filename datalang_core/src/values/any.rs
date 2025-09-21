use super::{Value, ValueFeatures};

#[derive(Debug, Clone)]
pub struct AnyValue;
#[derive(Debug, Clone)]
pub struct AnyInteger;
#[derive(Debug, Clone)]
pub struct AnyFloat;
#[derive(Debug, Clone)]
pub struct AnyString;
#[derive(Debug, Clone)]
pub struct AnyBool;
#[derive(Debug, Clone)]
pub struct AnyArray(pub Box<Value>, pub Box<Value>);
#[derive(Debug, Clone)]
pub struct AnyMap(pub Box<Value>);

impl ValueFeatures for AnyValue {
   fn select_key(&self, _key: &str) -> Option<&Value> {
       None
   }
}

impl ValueFeatures for AnyInteger {
   fn select_key(&self, _key: &str) -> Option<&Value> {
       None
   }
}

impl ValueFeatures for AnyFloat {
   fn select_key(&self, _key: &str) -> Option<&Value> {
       None
   }
}

impl ValueFeatures for AnyString {
   fn select_key(&self, _key: &str) -> Option<&Value> {
       None
   }
}

impl ValueFeatures for AnyBool {
   fn select_key(&self, _key: &str) -> Option<&Value> {
       None
   }
}

impl ValueFeatures for AnyArray {
   fn select_key(&self, key: &str) -> Option<&Value> {
      match key {
         "valueschema" => Some(&self.0),
         "length" => Some(&self.1),
         _ => None
      }
   }
}

impl ValueFeatures for AnyMap {
   fn select_key(&self, key: &str) -> Option<&Value> {
      match key {
         "valueschema" => Some(&self.0),
         _ => None
      }
   }
}
