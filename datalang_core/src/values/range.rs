use std::ops::Bound;
use super::{Value, ValueFeatures, KeySelectionError, IndexSelectionError};
use super::literal::{FloatLiteral, IntegerLiteral};
use crate::concrete::ConcreteValue;

#[derive(Debug, Clone)]
pub struct IntegerRange {
   pub start: Bound<i32>,
   pub end: Bound<i32>,

   start_repr: Option<Box<Value>>,
   end_repr: Option<Box<Value>>
}

#[derive(Debug, Clone)]
pub struct FloatRange {
   pub start: Bound<f32>,
   pub end: Bound<f32>,

   start_repr: Option<Box<Value>>,
   end_repr: Option<Box<Value>>
}

impl IntegerRange {
   pub fn new(start: Bound<i32>, end: Bound<i32>) -> Self {
      Self {
         start,
         end,
         start_repr: match start {
            Bound::Excluded(start) | Bound::Included(start) => Some(Box::new(Value::Integer(IntegerLiteral(start)))),
            Bound::Unbounded => None
         },
         end_repr: match end {
            Bound::Excluded(end) | Bound::Included(end) => Some(Box::new(Value::Integer(IntegerLiteral(end)))),
            Bound::Unbounded => None
         }
      }
   }
}

impl ValueFeatures for IntegerRange {
   fn select_key(&self, key: &str) -> Result<&Value, KeySelectionError> {
      match key {
         "start" => self.start_repr.as_ref().map(|v| &**v).ok_or(KeySelectionError::NoSuchKey),
         "end" => self.end_repr.as_ref().map(|v| &**v).ok_or(KeySelectionError::NoSuchKey),
         _ => Err(KeySelectionError::NoSuchKey)
      }
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      crate::range_utils::singleton_range(self.start, self.end).is_some()
   }

   fn concretize(self) -> Option<ConcreteValue> {
      Some(ConcreteValue::Integer(crate::range_utils::singleton_range(self.start, self.end)?))
   }
}

impl FloatRange {
   pub fn new(start: Bound<f32>, end: Bound<f32>) -> Self {
      Self {
         start,
         end,
         start_repr: match start {
            Bound::Excluded(start) | Bound::Included(start) => Some(Box::new(Value::Float(FloatLiteral(start)))),
            Bound::Unbounded => None
         },
         end_repr: match end {
            Bound::Excluded(end) | Bound::Included(end) => Some(Box::new(Value::Float(FloatLiteral(end)))),
            Bound::Unbounded => None
         }
      }
   }
}

impl ValueFeatures for FloatRange {
   fn select_key(&self, key: &str) -> Result<&Value, KeySelectionError> {
      match key {
         "start" => self.start_repr.as_ref().map(|v| &**v).ok_or(KeySelectionError::NoSuchKey),
         "end" => self.end_repr.as_ref().map(|v| &**v).ok_or(KeySelectionError::NoSuchKey),
         _ => Err(KeySelectionError::NoSuchKey)
      }
   }

   fn select_index(&self, _index: i32) -> Result<&Value, IndexSelectionError> {
      Err(IndexSelectionError::NotSelectable)
   }

   fn is_definite(&self) -> bool {
      crate::range_utils::singleton_range_float(self.start, self.end).is_some()
   }

   fn concretize(self) -> Option<ConcreteValue> {
      Some(ConcreteValue::Float(crate::range_utils::singleton_range_float(self.start, self.end)?))
   }
}
