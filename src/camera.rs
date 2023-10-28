use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
  commands.spawn((Camera2dBundle::default(), MainCamera));
}

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup);
  }
}
