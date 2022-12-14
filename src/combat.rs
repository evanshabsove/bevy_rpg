use bevy::{prelude::*, input::keyboard};

use crate::{AppState, enemy::Enemy, combat_stats::CombatStats, player::Player};

pub struct CombatPlugin;

pub struct FightEvent {
  pub(crate) target: Entity,
  pub(crate) damage_amount: isize
}

pub struct EnterCombatEvent {

}

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_event::<FightEvent>()
          .add_event::<EnterCombatEvent>()
        .add_system_set(
          SystemSet::on_update(AppState::Combat)
            .with_system(combat_camera)
            .with_system(damage_calculation)
        )
        .add_system_set(
          SystemSet::on_exit(AppState::Combat).with_system(despawn_enemy)
        )
        .add_system(enter_combat)
        .add_system(leave_combat);
    }
}

fn damage_calculation(
  mut fight_event: EventReader<FightEvent>,
  mut target_query: Query<&mut CombatStats>,
  mut state: ResMut<State<AppState>>
) {
  for event in fight_event.iter() {
    let mut target_stats = target_query.get_mut(event.target).expect("Fighting target without stats!");

    target_stats.health = std::cmp::max(
      target_stats.health - (event.damage_amount - target_stats.defense),
      0
    );

    if target_stats.health == 0 {
      state.set(AppState::OverWorld).unwrap();
    }
  }
}

fn despawn_enemy(
  mut commands: Commands,
  enemy_query: Query<Entity, With<Enemy>>
) {
  for entity in enemy_query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

fn combat_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
  let mut camera_transform = camera_query.single_mut();

  camera_transform.translation.x = 0.0;
  camera_transform.translation.y = 0.0;
}

fn enter_combat (
  mut enter_combat_event: EventReader<EnterCombatEvent>,
  mut keyboard: ResMut<Input<KeyCode>>,
  mut state: ResMut<State<AppState>>,
  mut commands: Commands,
  mut asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  for event in enter_combat_event.iter() {
    state.set(AppState::Combat).unwrap();

    let texture_handle = asset_server.load("mystic_woods_free_v0.2/sprites/characters/slime.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 5, 7, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
              transform: Transform {
                translation: Vec3::new(0.0,0.0, 900.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(CombatStats {
            health: 3,
            max_health: 3,
            attack: 2,
            defense: 1,
        });
  }
}

fn leave_combat(
  mut keyboard: ResMut<Input<KeyCode>>,
  mut state: ResMut<State<AppState>>
) {
  if keyboard.just_pressed(KeyCode::M) {
    state.set(AppState::OverWorld).unwrap();

    keyboard.clear()
  }
}