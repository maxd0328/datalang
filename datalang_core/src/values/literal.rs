use super::{IndexSelectionError, KeySelectionError, Value, ValueFeatures};
use crate::concrete::ConcreteValue;

#[derive(Debug, Clone)]
pub struct IntegerLiteral(pub i32);
#[derive(Debug, Clone)]
pub struct FloatLiteral(pub f32);
#[derive(Debug, Clone)]
pub struct StringLiteral(pub String);
#[derive(Debug, Clone)]
pub struct BoolLiteral(pub bool);
#[derive(Debug, Clone)]
pub struct Undefined;

impl ValueFeatures for IntegerLiteral {
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      true
   }

   fn concretize(self) -> Option<ConcreteValue> {
      Some(ConcreteValue::Integer(self.0))
   }
}

impl ValueFeatures for FloatLiteral {
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      true
   }

   fn concretize(self) -> Option<ConcreteValue> {
      Some(ConcreteValue::Float(self.0))
   }
}

impl ValueFeatures for StringLiteral {
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      true
   }

   fn concretize(self) -> Option<ConcreteValue> {
      Some(ConcreteValue::String(self.0))
   }
}

impl ValueFeatures for BoolLiteral {
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      true
   }

   fn concretize(self) -> Option<ConcreteValue> {
      Some(ConcreteValue::Bool(self.0))
   }
}

impl ValueFeatures for Undefined {
   fn select_key(&self, _key: &str) -> Result<&Value, KeySelectionError> {
      Err(KeySelectionError::NotSelectable)
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      true
   }

   fn concretize(self) -> Option<ConcreteValue> {
      Some(ConcreteValue::Undefined)
   }
}
