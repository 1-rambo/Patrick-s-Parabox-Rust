use bevy::prelude::*;
use bevy::color::palettes::*;

use crate::{ Level, GameState, despawn_screen, TEXT_COLOR };

pub fn game_plugin(app: &mut App) {
    app
        //.init_state::<GameState>()
        .add_systems(OnEnter(GameState::Game), game_setup)
        //.add_systems(Update, (game_action, button_system).run_if(in_state(GameState::Game)));
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

#[derive(Component)]
struct OnGameScreen;

fn game_setup(
    mut commands: Commands,
    _level: Res<Level>,
) {
    //println!("Setting up game screen");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,                
                ..default()
            },
            BackgroundColor(css::TURQUOISE.into()),
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },
                )
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn((
                        Text::new("In Game Screen"),
                        TextFont {
                            font_size: 80.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR.into())),
                    );
                });
        });
}