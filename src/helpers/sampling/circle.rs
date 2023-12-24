use std::f32::consts::PI;

use super::PointSampler;
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct CircleSampler {
  distance: f32,
}

impl CircleSampler {
  fn new(distance: f32) -> Self {
    Self {distance}
  }
}

impl Default for CircleSampler {
  fn default() -> Self {
    Self {distance: 10.0}
  }
}

impl PointSampler for CircleSampler {
  type Iter = CircleIter;
  fn samples(&self, num: usize) -> Self::Iter {
    Self::Iter {
      distance: self.distance,
      current: if num == 0 {2.0 * PI} else {0.0},
      delta: (2.0 * PI) / (num as f32),
    }
  }
}

pub struct CircleIter {
  distance: f32,
  current: f32,
  delta: f32
}

impl Iterator for CircleIter {
  type Item = Vec2;
  fn next(&mut self) -> Option<Self::Item> {
    if self.current == 2.0 * PI {
      return None;
    }
    let direction = Vec2::from_angle(self.current);
    self.current += self.delta;
    Some(direction * self.distance)
  }
}

