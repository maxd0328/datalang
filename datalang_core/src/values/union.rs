use super::{Value, ValueFeatures};

#[derive(Debug, Clone)]
pub struct Union(pub Vec<Value>);

impl ValueFeatures for Union {
   fn select_key(&self, _key: &str) -> Option<&Value> {
       None
   }
}
