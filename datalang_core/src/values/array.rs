use super::{Value, ValueFeatures};

#[derive(Debug, Clone)]
pub struct Array(pub Vec<Value>);

impl ValueFeatures for Array {
   fn select_key(&self, _key: &str) -> Option<&Value> {
       None
   }
}
