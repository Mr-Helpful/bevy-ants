use super::ant::Ant;
use super::food::FoodStore;
use crate::helpers::{MouseCoords, RectSensor, SpawnEvent};
use bevy::prelude::*;

const NEST_COLOR: Color = Color::RED;
const NEST_SCALE: Vec2 = Vec2::splat(10.0);

const START_FOOD: FoodStore = FoodStore(50);
const ANT_COST: FoodStore = FoodStore(5);

#[derive(Component, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct NestMarker;

#[derive(Bundle, Clone)]
pub struct Nest {
  marker: NestMarker,
  store: FoodStore,
  collider: RectSensor,
  sprite: SpriteBundle,
}

impl Default for Nest {
  fn default() -> Self {
    Self {
      marker: default(),
      store: START_FOOD,
      collider: RectSensor::from(NEST_SCALE),
      sprite: SpriteBundle {
        sprite: Sprite {
          color: NEST_COLOR,
          custom_size: Some(NEST_SCALE),
          ..default()
        },
        ..default()
      },
    }
  }
}

impl Nest {
  pub fn new(position: Vec2) -> Self {
    let mut nest = Self::default();
    nest.sprite.transform = Transform::from_translation(position.extend(0.0));
    nest
  }
}

fn spawn_nest(mut spawn_events: EventReader<SpawnEvent<Nest>>, mut commands: Commands) {
  for event in spawn_events.iter() {
    commands.spawn(Nest::new(event.pos()));
  }
}

fn spawn_nest_on_key(
  mut spawn_events: EventWriter<SpawnEvent<Nest>>,
  keys: Res<Input<KeyCode>>,
  coords: Res<MouseCoords>,
) {
  if keys.just_pressed(KeyCode::N) {
    spawn_events.send(SpawnEvent::from(coords.0))
  }
}

fn spawn_ants(
  mut spawn_events: EventWriter<SpawnEvent<Ant>>,
  mut query: Query<(&mut FoodStore, &Transform), With<NestMarker>>,
) {
  spawn_events.send_batch(query.iter_mut().filter_map(|(mut food, transform)| {
    (*food >= ANT_COST).then(|| {
      *food -= ANT_COST;
      SpawnEvent::from(transform.translation.truncate())
    })
  }));
}

/// Adds food to all nests in the simulation on F pressed
///
/// @todo change to clicking on a nest adding food instead
fn add_nest_food(keys: Res<Input<KeyCode>>, mut query: Query<&mut FoodStore, With<NestMarker>>) {
  if keys.just_pressed(KeyCode::F) {
    for mut food in &mut query {
      *food += FoodStore(1);
    }
  }
}

/// ## Overview
///
/// Allows nests within a simulation to keep track of the food that they
/// currently store and spawn Ants when they have sufficient food.
///
/// ## Examples
///
/// ```
/// # use bevy::prelude::*
/// #
/// # fn main() {
/// App::new().add_plugins(NestPlugin)
/// # }
/// ```
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct NestPlugin;

impl Plugin for NestPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<SpawnEvent<Nest>>().add_systems(
      Update,
      ((spawn_ants, add_nest_food), spawn_nest_on_key, spawn_nest).chain(),
    );
  }
}
