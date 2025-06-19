use bevy::prelude::*;
use bevy::color::palettes::*;

use crate::{ Level, GameState, despawn_screen, TEXT_COLOR };
use crate::configs::LevelConfig;

pub fn game_plugin(app: &mut App) {
    app
        //.init_state::<GameState>()
        .add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(FixedUpdate, (
            game_action, 
            button_system
        ).chain().run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
        .insert_resource(LevelConfig::new(1, "assets/levels/1.json"));
}

#[derive(Component)]
struct OnGameScreen;

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);
const STAY: (i32, i32) = (0, 0);

fn game_setup(
    mut commands: Commands,
    level_config: ResMut<LevelConfig>,
    asset_server: Res<AssetServer>,
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
            BackgroundColor(css::DARK_BLUE.into()),
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
                        Text::new(format!("{:?}", level_config.paraboxes[0])),
                        TextFont {
                            font_size: 80.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR.into())),
                    );
                });
        });
}

fn game_action(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,    
    query: Query<Entity, With<OnGameScreen>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level_config: ResMut<LevelConfig>,
) {
    let movement = if keyboard_input.just_pressed(KeyCode::ArrowLeft) || keyboard_input.just_pressed(KeyCode::KeyA) {
        println!("Move left");
        LEFT
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) || keyboard_input.just_pressed(KeyCode::KeyD) {
        println!("Move right");
        RIGHT
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
        println!("Move up");
        UP
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS) {
        println!("Move down");
        DOWN 
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        // Exit game
        game_state.set(GameState::Menu);
        STAY
    } else {
        // No movement
        STAY
    };
    if movement != STAY {
        level_config.shift(movement);
        // If there was a movement, we can despawn the current game screen
        for entity in &query {
            commands.entity(entity).despawn();
        }
        // And set up the new game screen
        game_setup(commands, level_config, asset_server);
    }
}

const GAME_NORMAL_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5); // Normal state: gray
const GAME_HOVERED_BUTTON: Color = Color::srgb(0.6, 0.6, 0.6); // Hovered state: slightly lighter gray
const GAME_HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.4, 0.6, 0.4); // Hovered and pressed state: greenish gray
const GAME_PRESSED_BUTTON: Color = Color::srgb(0.4, 0.7, 0.4); // Pressed state: more greenish gray

#[derive(Component)]
struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => GAME_PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => GAME_HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => GAME_HOVERED_BUTTON.into(),
            (Interaction::None, None) => GAME_NORMAL_BUTTON.into(),
        }
    }
}