#![allow(clippy::type_complexity)]
use std::f32::consts::PI;

use bevy::prelude::*;

mod helpers;
mod simulation;

use bevy_rapier2d::prelude::RapierPhysicsPlugin;
use helpers::{ArcSampler, CameraPlugin, CoordsPlugin};
use simulation::{AntPlugin, FoodPlugin, NestPlugin, PheremonePlugin};

#[derive(Component, Default)]
pub struct CanvasMarker;
pub const PHEREMONE_LAYER: u8 = 1;

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::BLACK))
    // Helpers / Camera management
    .add_plugins((DefaultPlugins, CameraPlugin, CoordsPlugin))
    .add_plugins(PheremonePlugin::<CanvasMarker>::new(PHEREMONE_LAYER, 2.0))
    // Libary Plugins
    .add_plugins(RapierPhysicsPlugin::<()>::default())
    // Ant Simulation
    .add_plugins((
      AntPlugin(None, ArcSampler::new(10.0, PI / 3.0)),
      NestPlugin,
      FoodPlugin,
    ))
    .run();
}
