use crate::values::Value;
use crate::values::literal::*;
use crate::values::union::*;

pub fn conforms(value: &Value, schema: &Value) -> bool {
   use Value as V;
   use crate::range_utils as ru;

   match (value, schema) {
      // Triviality of `any`
      (_, V::Any(_)) => true,

      // Conformity to any element of a union
      (value, V::Union(Union(union))) => union.iter().any(|schema| conforms(value, schema)),
      // Conformity to a schema by all elements in a union
      (V::Union(Union(union)), schema) => union.iter().all(|value| conforms(value, schema)),

      // Any integer
      (V::IntegerRange(_), V::AnyInteger(_)) => true,
      (V::Integer(_), V::AnyInteger(_)) => true,
      // Any float
      (V::FloatRange(_), V::AnyFloat(_)) => true,
      (V::Float(_), V::AnyFloat(_)) => true,
      // Any string
      (V::String(_), V::AnyString(_)) => true,
      // Any bool
      (V::Bool(_), V::AnyBool(_)) => true,

      // Integer falls within range
      (V::Integer(v), V::IntegerRange(range)) => ru::in_bounds(v.0, range.start, range.end),
      // Range contains only a single integer
      (V::IntegerRange(range), V::Integer(v)) => ru::singleton_range(range.start, range.end) == Some(v.0),
      // Integer range contained within a range
      (V::IntegerRange(value), V::IntegerRange(schema)) => {
         ru::range_in_bounds(value.start, value.end, schema.start, schema.end, 1)
      }
      // Two equivalent integers
      (V::Integer(value), V::Integer(schema)) => value.0 == schema.0,

      // Float falls within range
      (V::Float(v), V::FloatRange(range)) => ru::in_bounds(v.0, range.start, range.end),
      // Range contains only a single float
      (V::FloatRange(range), V::Float(v)) => ru::singleton_range_float(range.start, range.end) == Some(v.0),
      // Float range contained within a range
      (V::FloatRange(value), V::FloatRange(schema)) => {
         ru::range_in_bounds(value.start, value.end, schema.start, schema.end, 0.0)
      }
      // Two equivalent floats
      (V::Float(value), V::Float(schema)) => value.0 == schema.0,

      // Two equivalent strings
      (V::String(value), V::String(schema)) => value.0 == schema.0,

      // Two equivalent bools
      (V::Bool(value), V::Bool(schema)) => value.0 == schema.0,

      // Array description conforms to another array description
      (V::AnyArray(value), V::AnyArray(schema)) => conforms(&value.0, &schema.0) && conforms(&value.1, &schema.1),
      // Array conforms to array description
      (V::Array(value), V::AnyArray(schema)) => {
         conforms(&Value::Integer(IntegerLiteral(value.elems.len() as i32)), &schema.0)
            && value.elems.iter().all(|e| conforms(e, &schema.1))
      }
      // Array description conforms to array
      (V::AnyArray(value), V::Array(schema)) => {
         conforms(&value.0, &Value::Integer(IntegerLiteral(schema.elems.len() as i32)))
            && schema.elems.iter().all(|e| conforms(&value.1, e))
      }
      // Array conforms to another array
      (V::Array(value), V::Array(schema)) => {
         value.elems.len() == schema.elems.len()
            && value.elems.iter().zip(schema.elems.iter()).all(|(v, s)| conforms(v, s))
      }

      // Map description conforms to another map description
      (V::AnyMap(value), V::AnyMap(schema)) => conforms(&value.0, &schema.0),
      // Map conforms to map description (note a map description will never conform to a map)
      (V::Map(value), V::AnyMap(schema)) => value.entries.values().all(|v| conforms(v, &schema.0)),
      // Map conforms to another map
      (V::Map(value), V::Map(schema)) => {
         value.entries.len() == schema.entries.len()
            && value.entries.iter().all(|(k, vv)| schema.entries.get(k).map_or(false, |vs| conforms(vv, vs)))
      }

      // Nothing else conforms
      _ => false
   }
}
