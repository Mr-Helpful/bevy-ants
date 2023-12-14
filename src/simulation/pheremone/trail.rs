use bevy::{prelude::*, render::view::RenderLayers};

#[derive(Component, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct TrailMarker;

#[derive(Bundle, Clone, Default)]
pub struct Trail {
  marker: TrailMarker,
  layers: RenderLayers,
  sprite: SpriteBundle,
}

impl Trail {
  pub fn new(layer: u8, color: Color, size: Vec2) -> Self {
    Self {
      marker: TrailMarker,
      layers: RenderLayers::layer(layer),
      sprite: SpriteBundle {
        sprite: Sprite {
          color,
          custom_size: Some(size),
          ..default()
        },
        ..default()
      },
    }
  }
}
