use std::ops::Bound;
use super::{Value, ValueFeatures};
use super::literal::{FloatLiteral, IntegerLiteral};

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
   fn select_key(&self, key: &str) -> Option<&Value> {
      match key {
         "start" => self.start_repr.as_ref().map(|v| &**v),
         "end" => self.end_repr.as_ref().map(|v| &**v),
         _ => None
      }
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
   fn select_key(&self, key: &str) -> Option<&Value> {
      match key {
         "start" => self.start_repr.as_ref().map(|v| &**v),
         "end" => self.end_repr.as_ref().map(|v| &**v),
         _ => None
      }
   }
}
