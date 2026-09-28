use bevy::{
    prelude::{Commands, Component, Entity, Node, Query, With},
    ui::{AlignItems, FlexDirection, JustifyContent, Val},
};

use crate::menu::{ButtonAction, spawn_menu_button};

#[derive(Component)]
pub struct StartingMenu;

pub fn create(mut commands: Commands) {
    commands
        .spawn((
            StartingMenu,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            spawn_menu_button(parent, ButtonAction::Start, "Start");
            spawn_menu_button(parent, ButtonAction::Quit, "Quit");
        });
}

pub fn remove(mut commands: Commands, query: Query<Entity, With<StartingMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
