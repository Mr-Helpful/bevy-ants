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

impl VecClamp for Vec2 {
  fn clamp_length_range(self, range: impl InclusiveBound<f32>) -> Self {
    let min = range.start().unwrap_or(f32::MIN);
    let max = range.end().unwrap_or(f32::MAX);
    self.clamp_length(min, max)
  }
}
impl VecClamp for Vec3 {
  fn clamp_length_range(self, range: impl InclusiveBound<f32>) -> Self {
    let min = range.start().unwrap_or(f32::MIN);
    let max = range.end().unwrap_or(f32::MAX);
    self.clamp_length(min, max)
  }
}
impl VecClamp for Vec4 {
  fn clamp_length_range(self, range: impl InclusiveBound<f32>) -> Self {
    let min = range.start().unwrap_or(f32::MIN);
    let max = range.end().unwrap_or(f32::MAX);
    self.clamp_length(min, max)
  }
}
