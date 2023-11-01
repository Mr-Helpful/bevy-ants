#![allow(clippy::type_complexity)]
use bevy::prelude::*;

mod ant;
mod camera;
mod coords;
mod kinetic;
mod nest;

use ant::AntPlugin;
use camera::CameraPlugin;
use coords::CoordsPlugin;

fn main() {
  App::new()
    // Helpers / Camera management
    .add_plugins((DefaultPlugins, CameraPlugin, CoordsPlugin))
    // Ant Simulation
    .add_plugins(AntPlugin::default())
    .run();
}
