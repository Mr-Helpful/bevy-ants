use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle, Default, Clone)]
pub struct RectSensor {
  collider: Collider,
  sensor: Sensor,
}

impl From<Vec2> for RectSensor {
  fn from(value: Vec2) -> Self {
    Self {
      collider: Collider::cuboid(value.x / 2.0, value.y / 2.0),
      ..default()
    }
  }
}
