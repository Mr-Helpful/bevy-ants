use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Event, Clone, Copy, Default, Debug, PartialEq)]
pub struct SpawnEvent<T>(pub Vec2, PhantomData<T>);

impl<T> From<Vec2> for SpawnEvent<T> {
  fn from(value: Vec2) -> Self {
    SpawnEvent(value, PhantomData)
  }
}
impl<T> SpawnEvent<T> {
  pub fn pos(&self) -> Vec2 {
    self.0
  }
}
