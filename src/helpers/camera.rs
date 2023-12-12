use bevy::prelude::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct CameraPlugin;

#[derive(Component, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct MainCameraMarker;

#[derive(Bundle)]
pub struct MainCamera {
  marker: MainCameraMarker,
  camera: Camera2dBundle,
}

impl Default for MainCamera {
  fn default() -> Self {
    Self {
      marker: MainCameraMarker,
      camera: Camera2dBundle {
        camera: Camera {
          order: 1,
          ..default()
        },
        ..default()
      },
    }
  }
}

fn setup(mut commands: Commands) {
  commands.spawn(MainCamera::default());
}

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup);
  }
}
