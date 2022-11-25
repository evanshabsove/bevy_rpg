use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkEntity;
use bevy_ecs_ldtk::{prelude::GridCoords};

use crate::{TILE_SIZE, AppState};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct OverWorldEnemy;

pub struct EnemyPlugin;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct EnemySpawn;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct EnemySpawnEntity;

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct EnemySpawnBundle {
    enemy_spawn_entity: EnemySpawnEntity,
    #[grid_coords]
    grid_coords: GridCoords,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
      app
      .add_startup_system(spawn_overworld_enemys)
      .add_system_set(
        SystemSet::on_update(AppState::OverWorld).with_system(move_enemys_to_spawns)
      );
    }
}

fn spawn_overworld_enemys(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("mystic_woods_free_v0.2/sprites/characters/slime.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 5, 7);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);
  commands
      .spawn_bundle(SpriteSheetBundle {
          texture_atlas: texture_atlas_handle,
            transform: Transform {
              translation: Vec3::new(100.0,100.0, 900.0),
              ..Default::default()
          },
          ..default()
      })
      .insert(OverWorldEnemy);
}

fn move_enemys_to_spawns(
    mut enemy_query: Query<&mut Transform, With<OverWorldEnemy>>,
    enemy_spawn_query: Query<&GridCoords, Added<EnemySpawnEntity>>,
) {
    enemy_spawn_query.for_each(|&grid_coords| {
        let mut enemy_transform = enemy_query.single_mut();

        enemy_transform.translation.x = grid_coords.x as f32 * TILE_SIZE;
        enemy_transform.translation.y = grid_coords.y as f32 * TILE_SIZE;
    });
}