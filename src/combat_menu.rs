use bevy::{prelude::*, input::keyboard};
use bevy_inspector_egui::Inspectable;

use crate::{AppState, combat_stats::{CombatStats, self}, enemy::Enemy, combat::FightEvent, player::Player};

pub struct CombatMenuPlugin;

#[derive(Component, Inspectable)]
pub struct CombatMenu;

#[derive(Component, Inspectable)]
pub struct EnemyHealthText;

#[derive(Component, Inspectable)]
pub struct AttackButton;

#[derive(Component, Inspectable)]
pub struct RunButton;

impl Plugin for CombatMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
          SystemSet::on_enter(AppState::Combat)
            .with_system(build_combat_menu)
        )
        .add_system_set(
          SystemSet::on_exit(AppState::Combat).with_system(despawn_combat_menu)
        )
        .add_system_set(
          SystemSet::on_update(AppState::Combat)
            .with_system(update_enemy_health_text)
            .with_system(attack_button_system)
        );
    }
}

fn build_combat_menu(
    mut commands: Commands,
    ass: ResMut<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
) {
    let mut font = ass.load("fonts/BeBasNeue-Regular.ttf");
    clear_color.0 = Color::BLACK;
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            background_color: bevy::prelude::BackgroundColor(Color::BLACK),
            visibility: Visibility {
                is_visible: true,
                ..Visibility::default()
            },
            ..NodeBundle::default()
        })
        .insert(CombatMenu)
        .with_children(|parent| {
            parent.spawn_bundle(
              TextBundle::from_section(
                "Enemy Health:",
                TextStyle {
                  font: font.clone(),
                  font_size: 30.0,
                  color: Color::WHITE,
                }
              )
            ).insert(EnemyHealthText);

            // use MenuItem::*;
            spawn_button(parent, font.clone(), "Attack");
            spawn_button(parent, font.clone(), "Run");
        });
}

fn spawn_button(parent: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    let mut button = parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(10.),
                    height: Val::Px(30.),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
              },
              background_color: bevy::prelude::BackgroundColor(Color::GRAY),
            ..ButtonBundle::default()
        });
    if text == "Attack" {
        button.insert(AttackButton);
    }
    button.with_children(|parent| {
      parent.spawn_bundle(
        TextBundle::from_section(
          text.to_string(),
          TextStyle {
            font: font,
            font_size: 30.0,
            color: Color::WHITE,
          }
        )
      );
    });
}

fn despawn_combat_menu(
  mut commands: Commands,
  enemy_query: Query<Entity, With<CombatMenu>>
) {
  for entity in enemy_query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

fn update_enemy_health_text(
  mut text_query: Query<&mut Text, With<EnemyHealthText>>,
  mut enemy_query: Query<&mut CombatStats, With<Enemy>>,
) {
  let combat_stats = enemy_query.single_mut();
  for mut text in &mut text_query {
      text.sections[0].value = format!("Enemy Health: {}", combat_stats.health);
  }
}

fn attack_button_system(
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<AttackButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut fight_event: EventWriter<FightEvent>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<&CombatStats, With<Player>>
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let target = enemy_query.iter().next().unwrap();
        let player_stats = player_query.single();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Attacked!".to_string();
                fight_event.send(FightEvent { target, damage_amount: player_stats.attack });
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
              text.sections[0].value = "Attack".to_string();
            }
        }
    }
}