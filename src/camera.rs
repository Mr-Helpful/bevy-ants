use bevy::prelude::*;
use bevy_pancam::*;

pub struct CameraPlugin;

fn setup(mut commands: Commands) {
  commands
    .spawn(Camera2dBundle::default())
    .insert(PanCam::default());
}

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(PanCamPlugin).add_systems(Startup, setup);
  }
}
