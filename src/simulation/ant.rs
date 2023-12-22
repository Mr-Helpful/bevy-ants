use super::food::FoodStore;
use super::food::FOOD_COLOR;
use super::nest::NEST_COLOR;
use super::pheremone::Trail;
use crate::helpers::{ArcSampler, Kinetic, MouseCoords, PointSampler, RectSensor, SpawnEvent};
use crate::CanvasMarker;
use bevy::prelude::*;
use bevy_turborand::prelude::*;
use std::f32::consts::PI;
use std::ops::RangeInclusive;

const PHEREMONE_LAYER: u8 = 1;
const ANT_COLOR: Color = Color::BLUE;
const ANT_SCALE: Vec2 = Vec2::splat(2.0);

const ANT_SPEED_LIMITS: RangeInclusive<f32> = 20.0..=30.0;
const ANT_ACCEL_LIMITS: RangeInclusive<f32> = 25.0..=30.0;

const ANT_WANDER_STRENGTH: f32 = 0.2;

#[derive(Component, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct AntMarker;

#[derive(Component, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum AntState {
  #[default]
  Searching,
  // Backtracking, // For when we improve ant AI.
}

impl AntState {
  pub fn color(&self) -> Color {
    use AntState::*;
    match self {
      Searching => NEST_COLOR,
    }
  }

  pub fn follow(&self) -> Color {
    use AntState::*;
    match self {
      Searching => FOOD_COLOR,
    }
  }
}

#[derive(Bundle, Clone)]
pub struct Ant {
  marker: AntMarker,
  brain: AntState,
  food: FoodStore,
  rng: RngComponent,
  kinetic: Kinetic,
  collider: RectSensor,
  sprite: SpriteBundle,
}

impl Default for Ant {
  fn default() -> Self {
    Self {
      marker: default(),
      brain: default(),
      food: default(),
      rng: default(),
      kinetic: default(),
      collider: RectSensor::from(ANT_SCALE),
      sprite: SpriteBundle {
        sprite: Sprite {
          color: ANT_COLOR,
          custom_size: Some(ANT_SCALE),
          ..default()
        },
        ..default()
      },
    }
  }
}

impl Ant {
  pub fn new(position: Vec2, rng: &mut ResMut<GlobalRng>) -> Self {
    let mut ant = Self {
      rng: RngComponent::from(rng),
      kinetic: Kinetic::at(position),
      ..default()
    };
    ant.sprite.transform = ant.kinetic.transform();
    ant
  }
}

/// Spawns new Ants within the simulation.
/// Sets their velocity using a random source.
fn spawn_ants(
  mut commands: Commands,
  mut spawn_events: EventReader<SpawnEvent<Ant>>,
  mut rng: ResMut<GlobalRng>,
) {
  for event in spawn_events.read() {
    let ant = Ant::new(event.pos(), &mut rng);
    let trail = Trail::new(PHEREMONE_LAYER, ant.brain.color(), ANT_SCALE);

    commands.spawn(ant).with_children(|children| {
        children.spawn(trail);
      });
  }
}

/// Adds an ant at the cursor on click.
fn spawn_ant_on_key(
  mut spawn_events: EventWriter<SpawnEvent<Ant>>,
  keys: Res<Input<KeyCode>>,
  coords: Res<MouseCoords>,
) {
  if keys.just_pressed(KeyCode::A) {
    spawn_events.send(SpawnEvent::from(coords.0))
  }
}

fn random_wander(mut query: Query<(&mut Kinetic, &mut RngComponent), With<AntMarker>>) {
  for (mut kinetic, mut rng) in &mut query {
    let direction = Vec2::from_angle(2.0 * PI * rng.f32());
    kinetic.move_in(direction, ANT_WANDER_STRENGTH);
  }
}

/// Updates the position of all Ants in the simulation.
/// Currently MVP: Moves in a straight line.
///
/// @todo implement reaction to pheremones
/// @todo parallelise with rayon's par_iter_mut and atomic fetch_add or mutexes
fn move_ants(mut query: Query<(&mut Transform, &mut Kinetic), With<AntMarker>>, time: Res<Time>) {
  for (mut transform, mut kinetic) in &mut query {
    *transform = kinetic
      .step(time.delta_seconds(), ANT_SPEED_LIMITS, ANT_ACCEL_LIMITS)
      .zero_acceleration()
      .transform();
  }
}

/// Despawn ants outside the FOV.
fn despawn_ants(
  mut commands: Commands,
  query: Query<(Entity, &InheritedVisibility), With<AntMarker>>,
) {
  for (entity, visibility) in &query {
    if !visibility.get() {
      commands.entity(entity).despawn();
    }
  }
}

/// ## Overview
///
/// Moves ants and allows ants to be spawned in a simulation.
#[derive(Default, Debug)]
pub struct AntPlugin(Option<u64>);

impl From<u64> for AntPlugin {
  fn from(value: u64) -> Self {
    Self(Some(value))
  }
}

impl Plugin for AntPlugin {
  fn build(&self, app: &mut App) {
    // @todo add wall collisions to prevent ants escaping
    let mut rng_plugin = RngPlugin::default();
    if let Some(seed) = self.0 {
      rng_plugin = rng_plugin.with_rng_seed(seed);
    }

    app
      .add_plugins(rng_plugin)
      .add_plugins(PheremonePlugin::<AntCanvasMarker>::new(1, 2.0))
      .add_event::<SpawnEvent<Ant>>()
      .add_systems(
        Update,
        (
          // get the smallest no. ants spawned
          despawn_ants,
          // decide where to move for each ant
          (random_wander,),
          // run ant actions after deciding where to move
          (move_ants, spawn_ant_on_key, spawn_ants),
        )
          .chain(),
      );
  }
}
