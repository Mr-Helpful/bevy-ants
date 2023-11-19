use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::camera::MainCamera;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Clone, Copy, Default, Debug, PartialEq)]
pub struct MouseCoords(pub Vec2);

/// Conversion of mouse events to world coordinates
/// This is from the [bevy cheat book](https://bevy-cheatbook.github.io/cookbook/cursor2world.html?highlight=coordinate#convert-cursor-to-world-coordinates)
/// @warn assumes a single, primary window
fn simple_cursor_system(
  mut coords: ResMut<MouseCoords>,
  // query to get the window (so we can read the current cursor position)
  window_query: Query<&Window, With<PrimaryWindow>>,
  // query to get camera transform
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
  // both these queries will panic if != 1 window / camera
  let (camera, camera_transform) = camera_query.single();
  let window = window_query.single();

  // check if the cursor is inside the window and get its position
  // then, ask bevy to convert into world coordinates, and truncate to discard Z
  if let Some(ray) = window
    .cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
  {
    coords.0 = ray.origin.truncate();
  }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct CoordsPlugin;

impl Plugin for CoordsPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<MouseCoords>()
      .add_systems(Update, simple_cursor_system);
  }
}
