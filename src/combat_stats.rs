use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct CombatStats {
  pub health: isize,
  pub max_health: isize,
  pub attack: isize,
  pub defense: isize
}
