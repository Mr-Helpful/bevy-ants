#![allow(clippy::type_complexity)]
use std::f32::consts::PI;

use bevy::prelude::*;

mod helpers;
mod simulation;

use bevy_rapier2d::prelude::RapierPhysicsPlugin;
use simulation::{AntPlugin, FoodPlugin, NestPlugin, PheremonePlugin};

#[derive(Component, Default)]
pub struct CanvasMarker;
pub const PHEREMONE_LAYER: u8 = 1;

fn main() {
  App::new()
    // Helpers / Camera management
    .add_plugins((DefaultPlugins, CameraPlugin, CoordsPlugin))
    .add_plugins(PheremonePlugin::<CanvasMarker>::new(PHEREMONE_LAYER, 2.0))
    // Libary Plugins
    .add_plugins(RapierPhysicsPlugin::<()>::default())
    // Ant Simulation
    .add_plugins((AntPlugin::default(), NestPlugin, FoodPlugin))
    .run();
}
