use super::{Value, ValueFeatures, KeySelectionError, IndexSelectionError};
use super::union::Union;
use crate::concrete::ConcreteValue;

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
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      false
   }

   fn concretize(self) -> Option<ConcreteValue> {
      None
   }
}

impl ValueFeatures for AnyInteger {
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      false
   }

   fn concretize(self) -> Option<ConcreteValue> {
      None
   }
}

impl ValueFeatures for AnyFloat {
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      false
   }

   fn concretize(self) -> Option<ConcreteValue> {
      None
   }
}

impl ValueFeatures for AnyString {
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      false
   }

   fn concretize(self) -> Option<ConcreteValue> {
      None
   }
}

impl ValueFeatures for AnyBool {
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      false
   }

   fn concretize(self) -> Option<ConcreteValue> {
      None
   }
}

impl ValueFeatures for AnyArray {
   fn select_key(&self, key: &str) -> Result<&Value, KeySelectionError> {
      match key {
         "valueschema" => Ok(&self.0),
         "length" => Ok(&self.1),
         _ => Err(KeySelectionError::NotSelectable)
      }
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      false
   }

   fn concretize(self) -> Option<ConcreteValue> {
      None
   }
}

impl ValueFeatures for AnyMap {
   fn select_key(&self, key: &str) -> Result<&Value, KeySelectionError> {
      match key {
         "valueschema" => Ok(&self.0),
         _ => todo!()
      }
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      false
   }

   fn concretize(self) -> Option<ConcreteValue> {
      None
   }
}
