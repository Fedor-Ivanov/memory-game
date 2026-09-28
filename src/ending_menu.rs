use bevy::{
    prelude::{BackgroundColor, Color, Commands, Component, Entity, Node, Query, Res, Text, With},
    ui::{AlignItems, FlexDirection, JustifyContent, UiRect, Val},
};

use crate::game_grid::GameOver;
use crate::menu::{ButtonAction, spawn_menu_button};

#[derive(Component)]
pub struct EndingMenu;

pub fn create(
    mut commands: Commands,
    game_over: Res<GameOver>,
    existing: Query<(), With<EndingMenu>>,
) {
    if !game_over.visible || !existing.is_empty() {
        return;
    }

    commands
        .spawn((
            EndingMenu,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            // Затемнение всего экрана: поле под ним остаётся видно
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        ))
        .with_children(|overlay| {
            // Панель самого меню
            overlay
                .spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.),
                        padding: UiRect::all(Val::Px(30.)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.85)),
                ))
                .with_children(|panel| {
                    panel.spawn(Text::new("You win!"));
                    spawn_menu_button(panel, ButtonAction::Restart, "Restart");
                    spawn_menu_button(panel, ButtonAction::ToMainMenu, "To main menu");
                });
        });
}

pub fn remove(mut commands: Commands, query: Query<Entity, With<EndingMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
