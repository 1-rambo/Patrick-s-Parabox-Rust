use bevy::prelude::*;
use bevy::color::palettes::*;
// use bevy::window::WindowClosing;

use crate::plugins::menu;
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
        .insert_resource(KeyboardTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(Level(1))
        .insert_resource(LevelConfig::new(1, "assets/levels/1.json"));
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Resource)]
struct KeyboardTimer(Timer);

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);
const STAY: (i32, i32) = (0, 0);

fn game_setup(
    commands: Commands,
    level_settings: Res<Level>,
    mut level_config: ResMut<LevelConfig>,
    asset_server: Res<AssetServer>,
) {
    //println!("Setting up game screen");
    level_config.load(level_settings.0);
    render_game(commands, level_config, asset_server);
}

fn render_game(
    mut commands: Commands,
    level_config: ResMut<LevelConfig>,
    _asset_server: Res<AssetServer>,
) {
    //println!("Setting up game screen");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(css::DARK_BLUE.into()),
            OnGameScreen,
        ))
        .with_children(|parent| {
            let parabox_num = level_config.paraboxes.len();
            let colors = vec![
                css::DARK_RED, css::GREEN, css::DARK_BLUE, css::MAGENTA, css::ORANGE, css::PURPLE,
                css::PINK, css::LIGHT_GRAY
            ];
            for (id, parabox) in level_config.paraboxes.iter().enumerate() {
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0 / parabox_num as f32),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(colors[id].into()),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(format!("{:?}", id + 1)),
                        TextFont {
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR.into())
                    ));
                    parent.spawn((
                        Text::new(format!("{:?}", parabox)),
                        TextFont {
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR.into())),
                    );
                });
            }
        });
}

fn game_action(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<KeyboardTimer>,
    query: Query<Entity, With<OnGameScreen>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<menu::MenuState>>,
    mut level_config: ResMut<LevelConfig>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let movement = if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            //println!("Move left");
            LEFT
        } else if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            //println!("Move right");
            RIGHT
        } else if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            //println!("Move up");
            UP
        } else if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            //println!("Move down");
            DOWN 
        } else if keyboard_input.pressed(KeyCode::Escape) {
            // Exit game
            game_state.set(GameState::LevelSelect);
            menu_state.set(menu::MenuState::Levels);
            STAY
        } else {
            // No movement
            STAY
        };
        if movement != STAY {
            let success = level_config.shift(None, None, movement);
            let win = success && level_config.check_win();
            // If there was a movement, we can despawn the current game screen
            for entity in &query {
                commands.entity(entity).despawn();
            }
            // And set up the new game screen
            render_game(commands, level_config, asset_server);
            if win {
                // If the player won, we transition to the win state
                game_state.set(GameState::Win);
            }
        }
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