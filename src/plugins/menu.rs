use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::color::palettes::*;

use crate::{ Level, GameState, despawn_screen, TEXT_COLOR };

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>) 
        .add_systems(Update, (start_button.run_if(in_state(MenuState::Main)),))
        .add_systems(OnEnter(MenuState::Levels), level_select_menu_setup)
        .add_systems(OnExit(MenuState::Levels), despawn_screen::<OnLevelSelectScreen>)
        .add_systems(Update, level_button.run_if(in_state(MenuState::Levels)))
        .add_systems(Update, (menu_action, button_system).run_if(in_state(GameState::Menu)))
        .add_systems(Update, (menu_action, button_system).run_if(in_state(GameState::LevelSelect)));
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Levels,
    #[default]
    Disabled,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnLevelSelectScreen;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
pub enum MenuButtonAction {
    SelectLevel,
    // Settings,
    // SettingsDisplay,
    // SettingsSound,
    BackToMainMenu,
    // BackToSettings,
    // GoToHelp,
    Quit,
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn main_menu_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    // let button_style = (
    //     width: Val::Px(250.0),
    //     height: Val::Px(65.0),
    //     margin: UiRect::all(Val::Px(20.0)),
    //     justify_content: JustifyContent::Center,
    //     align_items: AlignItems::Center,
    // );
    // let button_icon_style = (
    //     width: Val::Px(30.0),
    //     // This takes the icons out of the flexbox flow, to be positioned exactly
    //     position_type: PositionType::Absolute,
    //     // The icon will be close to the left border of the button
    //     left: Val::Px(10.0),
    // );

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,                
                ..default()
            },
            BackgroundColor(css::CRIMSON.into()),
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(50.0)),
                        // background_color: Color::CRIMSON.into(),
                        ..default()
                    },
                )
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn((
                        Text::new("Patrick's Parabox"),
                        TextFont {
                            font_size: 80.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR.into())),
                        // .with_style(Style {
                        //     margin: UiRect::all(Val::Px(50.0)),
                        //     ..default()
                        // }),
                    );
                    parent
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect::all(Val::Px(20.0)),
                                width: Val::Px(250.0),
                                height: Val::Px(65.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                // background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::SelectLevel,
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Start"),
                                TextFont {
                                    font_size: 40.0,
                                    ..default()
                                },  
                                TextColor(TEXT_COLOR.into()),
                            ));
                        });
                    parent
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect::all(Val::Px(20.0)),
                                width: Val::Px(250.0),
                                height: Val::Px(65.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                // background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit, // Placeholder for future actions
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Exit"),
                                TextFont {
                                    font_size: 40.0,
                                    ..default()
                                },  
                                TextColor(TEXT_COLOR.into()),
                            ));
                        });
                });
        });
}

pub fn level_select_menu_setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
) {
    //println!("Setting up level select menu");
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
            OnLevelSelectScreen,
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
                        Text::new("Level Select"),
                        TextFont {
                            font_size: 80.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR.into())),
                    );
                    for i in 1..=3 {
                        parent.spawn(Node{
                            width: Val::Percent(100.0),
                            height: Val::Percent(20.0),
                            margin: UiRect::all(Val::Px(20.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        })
                        .with_children(|parent| {
                            for level_number in (4*i-3)..=(4*i) {
                                let level_picked = Level(level_number);
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        margin: UiRect::all(Val::Px(20.0)),
                                        width: Val::Px(250.0),
                                        height: Val::Px(65.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(NORMAL_BUTTON.into()),
                                    level_picked,
                                ));
                                entity.with_children(|parent| {
                                    parent.spawn((Text::new(format!("Level {}", level_picked.0)),
                                        TextFont {
                                            font_size: 40.0,
                                            ..default()
                                        },  
                                        TextColor(TEXT_COLOR.into()),
                                    ));
                                });
                            }
                        });    
                    }
                    parent.spawn(Node{
                        width: Val::Percent(100.0),
                        height: Val::Percent(20.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn((
                                Button,
                                Node {
                                    margin: UiRect::all(Val::Px(20.0)),
                                    width: Val::Px(350.0),
                                    height: Val::Px(65.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    // background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::BackToMainMenu,
                            ))
                            .with_children(|parent| {
                                parent.spawn((Text::new("Main Menu"),
                                    TextFont {
                                        font_size: 40.0,
                                        ..default()
                                    },  
                                    TextColor(TEXT_COLOR.into()),
                                ));
                            });
                        parent
                            .spawn((
                                Button,
                                Node {
                                    margin: UiRect::all(Val::Px(20.0)),
                                    width: Val::Px(350.0),
                                    height: Val::Px(65.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                MenuButtonAction::Quit,
                            ))
                            .with_children(|parent| {
                                parent.spawn((Text::new("Exit"),
                                    TextFont {
                                        font_size: 40.0,
                                        ..default()
                                    },  
                                    TextColor(TEXT_COLOR.into()),
                                ));
                            });
                    });
                });
        });
}

fn start_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            //println!("Start button pressed");
            menu_state.set(MenuState::Levels);
        }
    }
}

fn level_button(
    interaction_query: Query<(&Interaction, &Level), (Changed<Interaction>, With<Button>)>,
    mut level_setting: ResMut<Level>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, level_picked) in &interaction_query {
        if *interaction == Interaction::Pressed {
            *level_setting = *level_picked;
            game_state.set(GameState::Game);
            menu_state.set(MenuState::Disabled);
        }
    }
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::SelectLevel => {
                    game_state.set(GameState::LevelSelect);
                    menu_state.set(MenuState::Levels);
                    //menu_state.set(MenuState::Levels);
                }
                // MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                // MenuButtonAction::SettingsDisplay => {
                //     menu_state.set(MenuState::SettingsDisplay);
                // }
                // MenuButtonAction::SettingsSound => {
                //     menu_state.set(MenuState::SettingsSound);
                // }
                MenuButtonAction::BackToMainMenu => {
                    game_state.set(GameState::Menu);
                    menu_state.set(MenuState::Main);
                }
                // MenuButtonAction::BackToSettings => {
                //     menu_state.set(MenuState::Settings);
                // }
                // MenuButtonAction::GoToHelp => {
                //     menu_state.set(MenuState::Help);
                // }
            }
        }
    }
}