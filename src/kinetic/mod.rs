use bevy::prelude::*;
use std::fmt::Display;

mod limits;
use limits::VecClamp;

use self::limits::InclusiveBound;

#[derive(Component, Clone, Copy, Default)]
pub struct Kinetic {
  pub position: Vec2,
  pub velocity: Vec2,
  pub acceleration: Vec2,
}

impl Kinetic {
  pub fn at(position: Vec2) -> Self {
    Self {
      position,
      ..default()
    }
  }

  /// Adds the acceleration needed to achieve `velocity`
  pub fn move_to(&mut self, velocity: Vec2, strength: f32) -> &mut Self {
    let accel = (velocity - self.velocity).normalize_or_zero();
    self.acceleration += accel * strength;
    self
  }

  /// Adds the acceleration needed to move in `direction`
  ///
  /// # Arguments
  ///
  /// * `direction` - a unit vector specifying the direction to move towards
  pub fn move_in(&mut self, direction: Vec2, strength: f32) -> &mut Self {
    let vec_length = if self.velocity != Vec2::ZERO {
      self.velocity.length()
    } else {
      0.0
    };
    self.move_to(direction.normalize_or_zero() * vec_length, strength)
  }

  /// Resets the currently stored acceleration
  pub fn zero_acceleration(&mut self) -> &mut Self {
    self.acceleration *= 0.0;
    self
  }

  /// Performs single step of constant acceleration simulation
  pub fn step(
    &mut self,
    delta: f32,
    vel_range: impl InclusiveBound<f32>,
    acc_range: impl InclusiveBound<f32>,
  ) -> &mut Self {
    self.acceleration = self.acceleration.clamp_length_range(acc_range);
    self.velocity += self.acceleration * delta;
    self.velocity = self.velocity.clamp_length_range(vel_range);
    self.position += self.velocity * delta;
    self
  }

  pub fn transform(&self) -> Transform {
    let mut transform = Transform::default();
    transform.look_to(Vec3::Z, self.velocity.extend(0.0));
    transform.translation = self.position.extend(0.0);
    transform
  }
}

impl Display for Kinetic {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{{ Acc = {}, Vel = {}, Pos = {} }}",
      self.acceleration, self.velocity, self.position
    )
  }
}
