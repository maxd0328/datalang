use std::ops::{Bound, Add, Sub};

pub fn in_bounds<T: PartialOrd>(val: T, start: Bound<T>, end: Bound<T>) -> bool {
   let lower = match start {
      Bound::Included(bound) => val >= bound,
      Bound::Excluded(bound) => val > bound,
      Bound::Unbounded => true
   };
   let upper = match end {
      Bound::Included(bound) => val <= bound,
      Bound::Excluded(bound) => val < bound,
      Bound::Unbounded => true
   };
   lower && upper
}

pub fn singleton_range(start: Bound<i32>, end: Bound<i32>) -> Option<i32> {
   match (start, end) {
      (Bound::Included(s), Bound::Included(e)) if s == e => Some(s),
      (Bound::Included(s), Bound::Excluded(e)) if s == e - 1 => Some(s),
      (Bound::Excluded(s), Bound::Included(e)) if s == e - 1 => Some(e),
      (Bound::Excluded(s), Bound::Excluded(e)) if s == e - 2 => Some(s + 1),
      _ => None
   }
}

pub fn singleton_range_float(start: Bound<f32>, end: Bound<f32>) -> Option<f32> {
   match (start, end) {
      (Bound::Included(start), Bound::Included(end)) if start == end => Some(start),
      _ => None
   }
}

pub fn range_in_bounds<T: PartialOrd + Add<T, Output = T> + Sub<T, Output = T> + Copy>(
   start: Bound<T>,
   end: Bound<T>,
   range_start: Bound<T>,
   range_end: Bound<T>,
   step: T
) -> bool {
   let lower = match (start, range_start) {
      (_, Bound::Unbounded) => true,
      (Bound::Unbounded, _) => false,
      (Bound::Included(start), Bound::Included(range_start)) => start >= range_start,
      (Bound::Included(start), Bound::Excluded(range_start)) => start > range_start,
      (Bound::Excluded(start), Bound::Included(range_start)) => start >= range_start - step,
      (Bound::Excluded(start), Bound::Excluded(range_start)) => start >= range_start
   };
   let upper = match (end, range_end) {
      (_, Bound::Unbounded) => true,
      (Bound::Unbounded, _) => false,
      (Bound::Included(end), Bound::Included(range_end)) => end <= range_end,
      (Bound::Included(end), Bound::Excluded(range_end)) => end < range_end,
      (Bound::Excluded(end), Bound::Included(range_end)) => end <= range_end + step,
      (Bound::Excluded(end), Bound::Excluded(range_end)) => end <= range_end
   };
   lower && upper
}
