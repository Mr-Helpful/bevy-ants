use bevy::prelude::*;
use derive_more::Mul;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::f32::consts::PI;

#[derive(Component, Default, Deref, DerefMut, Mul, Clone, Copy)]
pub struct Velocity(Vec2);

impl Distribution<Velocity> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Velocity {
    Velocity(Vec2::from_angle(2.0 * PI * rng.gen::<f32>()))
  }
}
