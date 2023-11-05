use bevy::prelude::*;

use super::ant::Ant;
use super::food::FoodStore;
use crate::helpers::{MouseCoords, SpawnEvent};

const NEST_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const NEST_SCALE: Vec2 = Vec2::splat(10.0);

const START_FOOD: FoodStore = FoodStore(50);
const ANT_COST: FoodStore = FoodStore(5);

#[derive(Component, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct NestMarker;

#[derive(Bundle, Clone)]
pub struct Nest {
  marker: NestMarker,
  store: FoodStore,
  sprite: SpriteBundle,
}

impl Default for Nest {
  fn default() -> Self {
    Self {
      marker: default(),
      store: START_FOOD,
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
  for event in &mut spawn_events {
    commands.spawn(Nest::new(event.pos()));
  }
}

fn add_nest_on_click(
  mut spawn_events: EventWriter<SpawnEvent<Nest>>,
  mut mouse_events: EventReader<MouseButtonInput>,
  coords: Res<MouseCoords>,
) {
  spawn_events.send_batch(
    mouse_events
      .iter()
      .filter(|&&MouseButtonInput { button, state, .. }| {
        (state == ButtonState::Pressed) && (button == MouseButton::Right)
      })
      .map(|_| SpawnEvent::from(coords.0)),
  )
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
      ((spawn_ants, add_nest_food), add_nest_on_click, spawn_nest).chain(),
    );
  }
}
