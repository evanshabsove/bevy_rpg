use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::Inspectable;
use bevy_ecs_ldtk::LdtkEntity;
use bevy_ecs_ldtk::{prelude::GridCoords};

use crate::combat_stats::CombatStats;
use crate::enemy::OverWorldEnemy;
use crate::{TILE_SIZE, AppState};
use crate::collider::Collider;
use crate::combat::EnterCombatEvent;
pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system_set(
          SystemSet::on_enter(AppState::OverWorld).with_system(show_player).with_system(show_overworld_enemys)
        )
        .add_system_set(
          SystemSet::on_exit(AppState::OverWorld).with_system(hide_player).with_system(hide_overworld_enemys)
        )
        .add_system_set(
          SystemSet::on_update(AppState::OverWorld).with_system(move_player_to_spawn).with_system(player_movement)
        );
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PlayerSpawn;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PlayerSpawnEntity;

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct PlayerSpawnBundle {
    player_spawn_entity: PlayerSpawnEntity,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player/Character_004.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
              transform: Transform {
                translation: Vec3::new(0.0,0.0, 900.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(Collider)
        .insert(CombatStats {
            health: 6,
            max_health: 6,
            attack: 3,
            defense: 2,
        });
}

fn move_player_to_spawn(
    mut player_query: Query<&mut Transform, With<Player>>,
    player_spawn_query: Query<&GridCoords, Added<PlayerSpawnEntity>>,
) {
    player_spawn_query.for_each(|&grid_coords| {
        let mut player_transform = player_query.single_mut();

        player_transform.translation.x = grid_coords.x as f32 * TILE_SIZE;
        player_transform.translation.y = grid_coords.y as f32 * TILE_SIZE;
    });
}

fn player_movement(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    keyboard: ResMut<Input<KeyCode>>,
    wall_query: Query<&Transform, (Without<Player>, With<Collider>)>,
    overworld_enemy_query: Query<&Transform, (Without<Player>, With<OverWorldEnemy>)>,
    mut enter_combat_event: EventWriter<EnterCombatEvent>
) {
    let (mut transform, mut player) = player_query.single_mut();


    let mut x_delta = 0.0;
    let mut y_delta = 0.0;
    if keyboard.just_pressed(KeyCode::W) {
      y_delta += TILE_SIZE
    } else if keyboard.just_pressed(KeyCode::A) {
      x_delta -= TILE_SIZE
    } else if keyboard.just_pressed(KeyCode::D) {
      x_delta += TILE_SIZE
    } else if keyboard.just_pressed(KeyCode::S) {
      y_delta -= TILE_SIZE
    }

    let target = transform.translation + Vec3::new(x_delta, y_delta, 0.0);

    if enemy_collision_check(target, &overworld_enemy_query) {
      enter_combat_event.send(EnterCombatEvent {  });
    } else if wall_collision_check(target, &wall_query) {
      transform.translation = target;
    }
}

fn enemy_collision_check(
  target_player_pos: Vec3,
  overworld_enemy_query: &Query<&Transform, (Without<Player>, With<OverWorldEnemy>)>
) -> bool {
  for overworld_transform in overworld_enemy_query.iter() {
    let collision = collide(
      target_player_pos,
      Vec2::splat(TILE_SIZE * 0.9),
      overworld_transform.translation,
      Vec2::splat(TILE_SIZE * 0.9)
    );

    if collision.is_some() {
      return true;
    }
  }
  false
}

fn wall_collision_check(
  target_player_pos: Vec3,
  wall_query: &Query<&Transform, (Without<Player>, With<Collider>)>
) -> bool {
  for wall_transform in wall_query.iter() {
    let collision = collide(
      target_player_pos,
      Vec2::splat(TILE_SIZE * 0.9),
      wall_transform.translation,
      Vec2::splat(TILE_SIZE * 0.9)
    );

    if collision.is_some() {
      return false;
    }
  }
  true
}

fn hide_player(
  mut player_query: Query<&mut Visibility, With<Player>>,
  children_query: Query<&Children, With<Player>>,
  mut child_visibility_query: Query<&mut Visibility, Without<Player>>
) {
  let mut player_vis = player_query.single_mut();
  player_vis.is_visible = false;

  if let Ok(children) = children_query.get_single() {
    for child in children.iter() {
      if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
        child_vis.is_visible = false;
      }
    }
  }
}

fn hide_overworld_enemys(
  mut over_world_enemy_query: Query<&mut Visibility, With<OverWorldEnemy>>,
  children_query: Query<&Children, With<OverWorldEnemy>>,
  mut child_visibility_query: Query<&mut Visibility, Without<OverWorldEnemy>>
){
  for (mut over_world_enemy_vis) in &mut over_world_enemy_query{
    over_world_enemy_vis.is_visible = false;

    if let Ok(children) = children_query.get_single() {
      for child in children.iter() {
        if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
          child_vis.is_visible = false;
        }
      }
    }   
  }
}

fn show_player(
  mut player_query: Query<&mut Visibility, With<Player>>,
  children_query: Query<&Children, With<Player>>,
  mut child_visibility_query: Query<&mut Visibility, Without<Player>>
) {
  let mut player_vis = player_query.single_mut();
  player_vis.is_visible = true;

  if let Ok(children) = children_query.get_single() {
    for child in children.iter() {
      if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
        child_vis.is_visible = true;
      }
    }
  }
}

fn show_overworld_enemys(
  mut over_world_enemy_query: Query<&mut Visibility, With<OverWorldEnemy>>,
  children_query: Query<&Children, With<OverWorldEnemy>>,
  mut child_visibility_query: Query<&mut Visibility, Without<OverWorldEnemy>>
) {
  for (mut over_world_enemy_vis) in &mut over_world_enemy_query{
    over_world_enemy_vis.is_visible = true;

    if let Ok(children) = children_query.get_single() {
      for child in children.iter() {
        if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
          child_vis.is_visible = true;
        }
      }
    }   
  }  
}