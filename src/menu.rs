use bevy::{
    ecs::message::MessageWriter,
    prelude::{
        AppExit, BackgroundColor, Button, Changed, ChildSpawnerCommands, Color, Component,
        Interaction, NextState, Node, Query, ResMut, Text, With,
    },
    ui::{AlignItems, JustifyContent, Val},
};

use crate::game_state::GameState;

#[derive(Component)]
pub enum ButtonAction {
    Start,
    Quit,
    Restart,
    ToMainMenu,
}

pub fn button_system(
    mut exit: MessageWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            ButtonAction::Start => {
                next_state.set(GameState::Playing);
            }

            ButtonAction::Quit => {
                exit.write(AppExit::Success);
            }

            ButtonAction::Restart => {
                next_state.set(GameState::Playing);
            }

            ButtonAction::ToMainMenu => {
                next_state.set(GameState::Start);
            }
        }
    }
}

pub fn spawn_menu_button(parent: &mut ChildSpawnerCommands, action: ButtonAction, label: &str) {
    parent
        .spawn((
            Button,
            action,
            Node {
                width: Val::Px(200.),
                height: Val::Px(60.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        ))
        .with_child(Text::new(label.to_string()));
}
