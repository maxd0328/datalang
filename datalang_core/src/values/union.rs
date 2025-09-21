use super::{Value, ValueFeatures, KeySelectionError, IndexSelectionError};

#[derive(Debug, Clone)]
pub struct Union(pub Vec<Value>);

impl ValueFeatures for Union {
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      let first = self.0.first().expect("A union should have at least one element");
      self.0.iter().all(|v| v.is_definite()) && self.0.iter().skip(1).all(|v| crate::conformity::conforms(v, first))
   }

   fn concretize(self) -> Option<crate::concrete::ConcreteValue> {
      if !self.is_definite() {
         return None
      }
      self.0.into_iter().next().unwrap().concretize()
   }
}
