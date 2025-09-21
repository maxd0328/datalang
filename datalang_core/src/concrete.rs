use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ConcreteValue {
   Integer(i32),
   Float(f32),
   String(String),
   Bool(bool),
   Array(Vec<ConcreteValue>),
   Map(HashMap<String, ConcreteValue>),
   Undefined
}
