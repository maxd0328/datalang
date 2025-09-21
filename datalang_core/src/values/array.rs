use super::{IndexSelectionError, KeySelectionError, Value, ValueFeatures};
use super::union::Union;
use super::literal::IntegerLiteral;
use crate::concrete::ConcreteValue;

#[derive(Debug, Clone)]
pub struct Array {
   pub elems: Vec<Value>,
   valueschema: Box<Value>,
   length: Box<Value>
}

impl Array {
   pub fn new(elems: Vec<Value>) -> Self {
      let valueschema = Box::new(Value::Union(Union(elems.iter().map(|v| v.clone()).collect())));
      let length = Box::new(Value::Integer(IntegerLiteral(elems.len() as i32)));
      Array {
         elems,
         valueschema,
         length
      }
   }
}

impl ValueFeatures for Array {
   fn select_key(&self, key: &str) -> Result<&Value, KeySelectionError> {
      match key {
         "valueschema" => Ok(&self.valueschema),
         "length" => Ok(&self.length),
         _ => Err(KeySelectionError::NoSuchKey)
      }
   }

   fn select_index(&self, index: i32) -> Result<&Value, IndexSelectionError> {
      self
         .elems
         .get::<usize>(index.try_into().map_err(|_| IndexSelectionError::OutOfBounds(self.elems.len()))?)
         .ok_or(IndexSelectionError::OutOfBounds(self.elems.len()))
   }

   fn is_definite(&self) -> bool {
      self.elems.iter().all(|e| e.is_definite())
   }

   fn concretize(self) -> Option<ConcreteValue> {
      Some(ConcreteValue::Array(self.elems.into_iter().map(|e| e.concretize()).collect::<Option<_>>()?))
   }
}
