#![allow(clippy::type_complexity)]
use bevy::prelude::*;

mod helpers;
mod simulation;

use bevy_rapier2d::prelude::RapierPhysicsPlugin;
use helpers::{CameraPlugin, CoordsPlugin};
use simulation::{AntPlugin, FoodPlugin, NestPlugin};

fn main() {
  App::new()
    // Helpers / Camera management
    .add_plugins((DefaultPlugins, CameraPlugin, CoordsPlugin))
    // Libary Plugins
    .add_plugins(RapierPhysicsPlugin::<()>::default())
    // Ant Simulation
    .add_plugins((AntPlugin::default(), NestPlugin, FoodPlugin))
    .run();
}
