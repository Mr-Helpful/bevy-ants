use super::PointSampler;
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component)]
pub struct ArcSampler {
  distance: f32,
  angle: f32,
}

impl ArcSampler {
  pub const fn new(distance: f32, angle: f32) -> Self {
    Self { distance, angle }
  }
}

impl Default for ArcSampler {
  fn default() -> Self {
    Self {
      distance: 10.0,
      angle: 0.0,
    }
  }
}

impl PointSampler for ArcSampler {
  type Iter = ArcIter;
  fn samples(&self, num: usize) -> Self::Iter {
    ArcIter {
      distance: self.distance,
      current: if num > 1 { -self.angle / 2.0 } else { 0.0 },
      delta: self.angle / (num as f32 - 1.0),
      num,
    }
  }
}

pub struct ArcIter {
  distance: f32,
  current: f32,
  delta: f32,
  num: usize,
}

impl Iterator for ArcIter {
  type Item = Vec2;
  fn next(&mut self) -> Option<Self::Item> {
    if self.num == 0 {
      return None;
    }
    let direction = Vec2::from_angle(self.current);
    self.current += self.delta;
    self.num -= 1;
    Some(direction * self.distance)
  }
}
