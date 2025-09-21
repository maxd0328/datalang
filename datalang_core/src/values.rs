pub mod literal;
pub mod range;
pub mod array;
pub mod map;
pub mod any;
pub mod union;

use std::ops::Deref;

pub trait ValueFeatures {
   fn select_key(&self, key: &str) -> Option<&Value>;
}

#[derive(Debug, Clone)]
pub enum Value {
   Integer(literal::IntegerLiteral),
   Float(literal::FloatLiteral),
   String(literal::StringLiteral),
   Bool(literal::BoolLiteral),
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

impl Deref for Value {
   type Target = dyn ValueFeatures;
   fn deref(&self) -> &Self::Target {
      match self {
         Self::Integer(value) => value,
         Self::Float(value) => value,
         Self::String(value) => value,
         Self::Bool(value) => value,
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
