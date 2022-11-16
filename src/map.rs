use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::AppState;

pub struct MapPlugin;

#[derive(Component)]
pub struct Map;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_map)
        .add_system_set(
          SystemSet::on_enter(AppState::OverWorld).with_system(show_map)
        )
        .add_system_set(
          SystemSet::on_exit(AppState::OverWorld).with_system(hide_map)
        );
    }
}

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("Untitled.ldtk"),
        ..Default::default()
    }).insert(Map);
}

fn hide_map(
  mut map_query: Query<&mut Visibility, With<Map>>,
  children_query: Query<&Children, With<Map>>,
  mut child_visibility_query: Query<&mut Visibility, Without<Map>>
) {
  let mut map_vis = map_query.single_mut();
  map_vis.is_visible = false;

  if let Ok(children) = children_query.get_single() {
    for child in children.iter() {
      if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
        child_vis.is_visible = false;
      }
    }
  }
}

fn show_map(
  mut map_query: Query<&mut Visibility, With<Map>>,
  children_query: Query<&Children, With<Map>>,
  mut child_visibility_query: Query<&mut Visibility, Without<Map>>
) {
  let mut map_vis = map_query.single_mut();
  map_vis.is_visible = true;

  if let Ok(children) = children_query.get_single() {
    for child in children.iter() {
      if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
        child_vis.is_visible = true;
      }
    }
  }
}