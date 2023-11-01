#![allow(clippy::type_complexity)]
use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;

mod ant;
mod camera;
mod coords;
mod kinetic;
mod nest;
mod velocity;

use ant::AntPlugin;
use camera::CameraPlugin;
use coords::CoordsPlugin;

fn main() {
  App::new()
    // Defaults
    .add_plugins(DefaultPlugins)
    // Helpers / View management
    .add_plugins((CameraPlugin, CoordsPlugin))
    // Ant Simulation
    .add_plugins(AntPlugin::<ChaCha8Rng>::default())
    .run();
}
