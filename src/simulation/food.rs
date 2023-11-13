use crate::helpers::{MouseCoords, RectSensor, SpawnEvent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierContext;
use derive_more::{AddAssign, SubAssign};

const FOOD_COLOR: Color = Color::GREEN;
const FOOD_SCALE: Vec2 = Vec2::splat(5.0);

const START_FOOD: FoodStore = FoodStore(5);

#[derive(Component, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct FoodMarker;

#[derive(Component, AddAssign, SubAssign, PartialEq, PartialOrd, Clone, Copy, Default, Debug)]
pub struct FoodStore(pub u16);

#[derive(Bundle, Clone)]
pub struct Food {
  marker: FoodMarker,
  store: FoodStore,
  collider: RectSensor,
  sprite: SpriteBundle,
}

impl Default for Food {
  fn default() -> Self {
    Self {
      marker: default(),
      store: START_FOOD,
      collider: RectSensor::from(FOOD_SCALE),
      sprite: SpriteBundle {
        sprite: Sprite {
          color: FOOD_COLOR,
          custom_size: Some(FOOD_SCALE),
          ..default()
        },
        ..default()
      },
    }
  }
}

impl Food {
  pub fn new(position: Vec2) -> Self {
    let mut food = Self::default();
    food.sprite.transform = Transform::from_translation(position.extend(0.0));
    food
  }
}

//

fn spawn_food(mut spawn_events: EventReader<SpawnEvent<Food>>, mut commands: Commands) {
  for event in spawn_events.iter() {
    commands.spawn(Food::new(event.pos()));
  }
}

fn spawn_food_on_keypress(
  mut spawn_events: EventWriter<SpawnEvent<Food>>,
  keys: Res<Input<KeyCode>>,
  coords: Res<MouseCoords>,
) {
  if keys.just_pressed(KeyCode::F) {
    spawn_events.send(SpawnEvent::from(coords.0))
  }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct FoodPlugin;

impl Plugin for FoodPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<SpawnEvent<Food>>()
      .add_systems(Update, (spawn_food, spawn_food_on_keypress));
  }
}
