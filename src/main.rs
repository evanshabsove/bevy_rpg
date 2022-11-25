use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 16.0;

mod combat;
mod combat_menu;
mod combat_stats;
mod collider;
mod enemy;
mod map;
mod systems;
mod player;
mod wall;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    OverWorld,
    Combat
}

fn main() {
    App::new()
        .add_state(AppState::OverWorld)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LdtkPlugin)
        .add_startup_system(systems::spawn_camera)
        .insert_resource(LevelSelection::Index(0))
        .add_plugin(map::MapPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(combat::CombatPlugin)
        .add_plugin(combat_menu::CombatMenuPlugin)
        .register_ldtk_entity::<player::PlayerSpawnBundle>("Player_spawn")
        .register_ldtk_entity::<enemy::EnemySpawnBundle>("Enemy_Spawn")
        .register_ldtk_int_cell::<wall::WallBundle>(1)
        .run();
}