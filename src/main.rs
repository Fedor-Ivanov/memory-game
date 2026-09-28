mod ending_menu;
mod game_grid;
mod game_state;
mod menu;
mod primary_window;
mod starting_menu;
use bevy::{
    camera::Camera2d,
    ecs::schedule::{IntoScheduleConfigs, common_conditions::resource_changed},
    picking::mesh_picking::MeshPickingPlugin,
    prelude::{
        App, Commands, DefaultPlugins, OnEnter, OnExit, PluginGroup, Startup, Update, in_state,
    },
    state::app::AppExtStates,
};

use game_state::GameState;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(primary_window::primary_window()),
        MeshPickingPlugin,
    ))
    .init_state::<GameState>()
    .init_resource::<game_grid::GameOver>()
    .init_resource::<game_grid::MismatchTimer>()
    .add_systems(Startup, setup)
    .add_systems(OnEnter(GameState::Start), starting_menu::create)
    .add_systems(OnExit(GameState::Start), starting_menu::remove)
    .add_systems(
        OnEnter(GameState::Playing),
        (game_grid::reset_mismatch_timer, game_grid::create),
    )
    .add_systems(
        OnExit(GameState::Playing),
        (
            game_grid::remove,
            ending_menu::remove,
            game_grid::reset_game_over,
        ),
    )
    .add_systems(Update, menu::button_system)
    .add_systems(
        Update,
        (
            game_grid::check_pair,
            game_grid::check_game_over,
            game_grid::handle_item_change,
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        Update,
        ending_menu::create.run_if(resource_changed::<game_grid::GameOver>),
    );

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
