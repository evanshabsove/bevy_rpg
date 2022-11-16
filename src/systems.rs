use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

pub fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        ..default()
    };

    commands.spawn_bundle(camera).insert(Camera);
}