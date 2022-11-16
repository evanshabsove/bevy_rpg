use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;

use crate::collider::Collider;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
    collider: Collider
}
