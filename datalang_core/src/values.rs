pub mod literal;
pub mod range;
pub mod array;
pub mod map;
pub mod any;
pub mod union;

use std::ops::Deref;
use crate::concrete::ConcreteValue;

pub trait ValueFeatures {
   fn select_key(&self, key: &str) -> Result<&Value, KeySelectionError>;
   fn select_index(&self, index: i32) -> Result<&Value, IndexSelectionError>;

   fn is_definite(&self) -> bool;
   fn concretize(self) -> Option<ConcreteValue>;
}

#[derive(Debug, Clone)]
pub enum Value {
   Integer(literal::IntegerLiteral),
   Float(literal::FloatLiteral),
   String(literal::StringLiteral),
   Bool(literal::BoolLiteral),
   Undefined(literal::Undefined),
   Array(array::Array),
   Map(map::Map),

   IntegerRange(range::IntegerRange),
   FloatRange(range::FloatRange),

   Any(any::AnyValue),
   AnyInteger(any::AnyInteger),
   AnyFloat(any::AnyFloat),
   AnyString(any::AnyString),
   AnyBool(any::AnyBool),
   AnyArray(any::AnyArray),
   AnyMap(any::AnyMap),

   Union(union::Union)
}

#[derive(Debug, Clone)]
pub enum KeySelectionError {
   NotSelectable,
   NoSuchKey
}

#[derive(Debug, Clone)]
pub enum IndexSelectionError {
   NotSelectable,
   OutOfBounds(usize)
}

impl Deref for Value {
   type Target = dyn ValueFeatures;
   fn deref(&self) -> &Self::Target {
      match self {
         Self::Integer(value) => value,
         Self::Float(value) => value,
         Self::String(value) => value,
         Self::Bool(value) => value,
         Self::Undefined(value) => value,
         Self::Array(value) => value,
         Self::Map(value) => value,

         Self::IntegerRange(value) => value,
         Self::FloatRange(value) => value,

         Self::Any(value) => value,
         Self::AnyInteger(value) => value,
         Self::AnyFloat(value) => value,
         Self::AnyString(value) => value,
         Self::AnyBool(value) => value,
         Self::AnyArray(value) => value,
         Self::AnyMap(value) => value,

         Self::Union(value) => value
      }
   }
}

impl Value {
   pub fn concretize(self) -> Option<ConcreteValue> {
      match self {
         Self::Integer(value) => value.concretize(),
         Self::Float(value) => value.concretize(),
         Self::String(value) => value.concretize(),
         Self::Bool(value) => value.concretize(),
         Self::Undefined(value) => value.concretize(),
         Self::Array(value) => value.concretize(),
         Self::Map(value) => value.concretize(),

         Self::IntegerRange(value) => value.concretize(),
         Self::FloatRange(value) => value.concretize(),

         Self::Any(value) => value.concretize(),
         Self::AnyInteger(value) => value.concretize(),
         Self::AnyFloat(value) => value.concretize(),
         Self::AnyString(value) => value.concretize(),
         Self::AnyBool(value) => value.concretize(),
         Self::AnyArray(value) => value.concretize(),
         Self::AnyMap(value) => value.concretize(),

         Self::Union(value) => value.concretize()
      }
   }
}

impl PartialEq for Value {
   fn eq(&self, other: &Value) -> bool {
      crate::conformity::conforms(self, other) && crate::conformity::conforms(other, self)
   }
}
