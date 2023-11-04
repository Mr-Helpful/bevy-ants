use std::ops::{RangeFrom, RangeFull, RangeInclusive, RangeToInclusive};

use bevy::prelude::*;

pub trait InclusiveBound<T> {
  fn start(&self) -> Option<T>;
  fn end(&self) -> Option<T>;
}

impl<T> InclusiveBound<T> for RangeFull {
  #[inline(always)]
  fn start(&self) -> Option<T> {
    None
  }
  #[inline(always)]
  fn end(&self) -> Option<T> {
    None
  }
}

impl<T: Copy> InclusiveBound<T> for RangeFrom<T> {
  #[inline(always)]
  fn start(&self) -> Option<T> {
    Some(self.start)
  }
  #[inline(always)]
  fn end(&self) -> Option<T> {
    None
  }
}

impl<T: Copy> InclusiveBound<T> for RangeToInclusive<T> {
  #[inline(always)]
  fn start(&self) -> Option<T> {
    None
  }
  #[inline(always)]
  fn end(&self) -> Option<T> {
    Some(self.end)
  }
}

impl<T: Copy> InclusiveBound<T> for RangeInclusive<T> {
  #[inline(always)]
  fn start(&self) -> Option<T> {
    Some(*self.start())
  }
  #[inline(always)]
  fn end(&self) -> Option<T> {
    Some(*self.end())
  }
}

/// Marker trait for Vecs with clamp_length methods
pub trait VecClamp {
  fn clamp_length_range(self, range: impl InclusiveBound<f32>) -> Self;
}

// these implementations are identical, so I'll use a macro.
macro_rules! impl_VecClamp {
  ($vec_name:ty) => {
    impl VecClamp for $vec_name {
      fn clamp_length_range(self, range: impl InclusiveBound<f32>) -> Self {
        if self == Self::ZERO {
          return self;
        }
        let min = range.start().unwrap_or(f32::MIN);
        let max = range.end().unwrap_or(f32::MAX);
        self.clamp_length(min, max)
      }
    }
  };
}
impl_VecClamp!(Vec2);
impl_VecClamp!(Vec3);
impl_VecClamp!(Vec4);
