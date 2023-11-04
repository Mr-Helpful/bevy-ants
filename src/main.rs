#![allow(clippy::type_complexity)]
use bevy::prelude::*;

mod helpers;
mod simulation;

use helpers::{CameraPlugin, CoordsPlugin};
use simulation::{AntPlugin, NestPlugin};

fn main() {
  App::new()
    // Helpers / Camera management
    .add_plugins((DefaultPlugins, CameraPlugin, CoordsPlugin))
    // Ant Simulation
    .add_plugins((AntPlugin::default(), NestPlugin))
    .run();
}
