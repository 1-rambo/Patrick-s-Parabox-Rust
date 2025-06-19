mod plugins;
mod configs;

use bevy::prelude::*;
use plugins::{menu, game, win};

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Level(u32);

fn main() {
    // use configs::*;
    // let mut level1 = LevelConfig::new(1, "assets/levels/1.json");
    // let up = (-1, 0);
    // let down = (1, 0);
    // let left = (0, -1);
    // let right = (0, 1);
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((
            menu::menu_plugin,
            // level_select::level_select_plugin,
            game::game_plugin,
            win::win_plugin,
        ))
        .insert_resource(Level(0))
        .run();
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    LevelSelect,
    Game,
    Win,
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2d::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}