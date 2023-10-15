use bevy::prelude::*;

mod ant;
mod camera;
mod nest;

use camera::CameraPlugin;

fn main() {
  App::new().add_plugins((DefaultPlugins, CameraPlugin)).run();
}
