use bevy::prelude::*;
use derive_more::{AddAssign, SubAssign};

use crate::ant::SpawnAntEvent;

const NEST_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const ANT_COST: Food = Food(5);

#[derive(Component, Clone, Copy, Default)]
pub struct NestMarker;

#[derive(Component, AddAssign, SubAssign, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Food(u16);

#[derive(Bundle, Clone)]
pub struct Nest {
  marker: NestMarker,
  food: Food,
  sprite: SpriteBundle,
}

impl Default for Nest {
  fn default() -> Self {
    Self {
      marker: default(),
      food: default(),
      sprite: SpriteBundle {
        sprite: Sprite {
          color: NEST_COLOR,
          ..default()
        },
        ..default()
      },
    }
  }
}

// impl Nest {
//   pub fn new(position: Vec2) -> Self {
//     let mut nest = Self::default();
//     nest.sprite.transform = Transform::from_translation(position.extend(0.0));
//     nest
//   }
// }

fn spawn_ants(
  mut spawn_events: EventWriter<SpawnAntEvent>,
  mut query: Query<(&mut Food, &Transform), (With<NestMarker>, Changed<Food>)>,
) {
  spawn_events.send_batch(query.iter_mut().filter_map(|(mut food, transform)| {
    (*food >= ANT_COST).then(|| {
      *food -= ANT_COST;
      SpawnAntEvent(transform.translation.truncate())
    })
  }));
}

/// Adds food to all nests in the simulation on F pressed
///
/// @todo change to clicking on a nest adding food instead
fn add_nest_food(keys: Res<Input<KeyCode>>, mut query: Query<&mut Food, With<NestMarker>>) {
  if keys.just_pressed(KeyCode::F) {
    for mut food in &mut query {
      *food += Food(1);
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
#[derive(Default)]
pub struct NestPlugin;

impl Plugin for NestPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (spawn_ants, add_nest_food));
  }
}
