use std::{marker::PhantomData, ops::DerefMut};

use bevy::input::{mouse::MouseButtonInput, ButtonState};
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand::Rng;

use crate::coords::MouseCoords;
use crate::velocity::Velocity;

const ANT_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
const ANT_SCALE: Vec2 = Vec2::splat(5.0);
const ANT_SPEED: f32 = 20.0;

#[derive(Component, Clone, Copy, Default)]
pub struct AntMarker;

#[derive(Component, Clone, Copy, Default)]
pub enum AntState {
  #[default]
  Searching,
  // Backtracking, // For when we improve ant AI.
}

#[derive(Bundle, Clone)]
pub struct Ant {
  marker: AntMarker,
  brain: AntState,
  velocity: Velocity,
  sprite: SpriteBundle,
}

impl Default for Ant {
  fn default() -> Self {
    Self {
      marker: default(),
      brain: default(),
      velocity: default(),
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
  pub fn new<R: DerefMut>(position: Vec2, rng: &mut R) -> Self
  where
    R::Target: SeedableEntropySource,
  {
    let velocity = rng.gen::<Velocity>() * ANT_SPEED;
    let mut transform = Transform::default();
    transform.look_to(Vec3::Z, velocity.extend(0.0));
    transform.translation = position.extend(0.0);

    let mut ant = Self::default();
    ant.sprite.transform = transform;
    ant.velocity = velocity;
    ant
  }
}

/// An event to spawn a new Ant in the simulation.
#[derive(Event, Clone, Copy)]
pub struct SpawnAntEvent(pub Vec2);

/// Spawns new Ants within the simulation.
/// Sets their velocity using a random source.
fn spawn_ants<R: SeedableEntropySource>(
  mut commands: Commands,
  mut spawn_events: EventReader<SpawnAntEvent>,
  mut rng: ResMut<GlobalEntropy<R>>,
) {
  for SpawnAntEvent(position) in &mut spawn_events {
    commands.spawn(Ant::new(*position, &mut rng));
  }
}

/// Adds an ant at the cursor on click.
fn add_ant(
  mut spawn_event: EventWriter<SpawnAntEvent>,
  mut mouse_events: EventReader<MouseButtonInput>,
  coords: Res<MouseCoords>,
) {
  spawn_event.send_batch(
    mouse_events
      .iter()
      .filter(|&&MouseButtonInput { state, button, .. }| {
        (state == ButtonState::Pressed) & (button == MouseButton::Left)
      })
      .map(|_| SpawnAntEvent(coords.0)),
  )
}

/// Updates the position of all Ants in the simulation.
/// Currently MVP: Moves in a straight line.
///
/// @todo implement reaction to pheremones
/// @todo parallelise with rayon's par_iter_mut and atomic fetch_add or mutexes
fn move_ants(mut query: Query<(&mut Transform, &Velocity), With<AntState>>, time: Res<Time>) {
  for (mut transform, velocity) in &mut query {
    transform.translation += velocity.extend(0.0) * time.delta_seconds();
  }
}

/// Despawn ants outside the FOV.
fn despawn_ants(
  mut commands: Commands,
  query: Query<(Entity, &ComputedVisibility), With<AntMarker>>,
) {
  for (entity, visibility) in &query {
    if !visibility.is_visible() {
      commands.entity(entity).despawn();
    }
  }
}

/// ## Overview
///
/// Moves ants and allows ants to be spawned in a simulation.
pub struct AntPlugin<R: SeedableEntropySource>(PhantomData<R>);

impl<R: SeedableEntropySource> Default for AntPlugin<R> {
  fn default() -> Self {
    Self(PhantomData)
  }
}

impl<R: SeedableEntropySource> Plugin for AntPlugin<R>
where
  R::Seed: Send + Sync + Copy,
{
  fn build(&self, app: &mut App) {
    // @todo add wall collisions to prevent ants escaping
    app
      .add_plugins(EntropyPlugin::<R>::default())
      .add_event::<SpawnAntEvent>()
      .add_systems(Update, add_ant)
      .add_systems(Update, move_ants)
      .add_systems(Update, despawn_ants)
      .add_systems(Update, spawn_ants::<R>);
  }
}
