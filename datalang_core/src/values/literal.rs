use super::{Value, ValueFeatures};

#[derive(Debug, Clone)]
pub struct IntegerLiteral(pub i32);
#[derive(Debug, Clone)]
pub struct FloatLiteral(pub f32);
#[derive(Debug, Clone)]
pub struct StringLiteral(pub String);
#[derive(Debug, Clone)]
pub struct BoolLiteral(pub bool);

impl ValueFeatures for IntegerLiteral {
   fn select_key(&self, _key: &str) -> Option<&Value> {
      None
   }
}

impl ValueFeatures for FloatLiteral {
   fn select_key(&self, _key: &str) -> Option<&Value> {
      None
   }
}

impl ValueFeatures for StringLiteral {
   fn select_key(&self, _key: &str) -> Option<&Value> {
      None
   }
}

impl ValueFeatures for BoolLiteral {
   fn select_key(&self, _key: &str) -> Option<&Value> {
      None
   }
}
