use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::color::palettes::basic::*;

use super::super::{ GameState, despawn_screen, TEXT_COLOR };

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(
            Update, 
            (menu_action, button_system).run_if(in_state(GameState::Menu)),
        );
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

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const MAIN_BACKGROUND: Color = Color::srgb(0.5, 0.5, 0.5);
// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    SelectLevel,
    // Settings,
    // SettingsDisplay,
    // SettingsSound,
    BackToMainMenu,
    BackToSettings,
    // GoToHelp,
    Quit,
}

fn button_system(
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

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    let button_text_style = (
        TextFont {
            font_size: 40.0,
            ..default()
        },  
        TextColor(TEXT_COLOR.into())
    );

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,                
                ..default()
            },
            BackgroundColor(MAIN_BACKGROUND.into()),
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
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::SelectLevel => {
                    menu_state.set(MenuState::Levels);
                }
                // MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                // MenuButtonAction::SettingsDisplay => {
                //     menu_state.set(MenuState::SettingsDisplay);
                // }
                // MenuButtonAction::SettingsSound => {
                //     menu_state.set(MenuState::SettingsSound);
                // }
                // MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                // MenuButtonAction::BackToSettings => {
                //     menu_state.set(MenuState::Settings);
                // }
                // MenuButtonAction::GoToHelp => {
                //     menu_state.set(MenuState::Help);
                // }
                _ => {}
            }
        }
    }
}