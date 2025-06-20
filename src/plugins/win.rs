use bevy::prelude::*;
use bevy::color::palettes::*;

use crate::{ GameState, despawn_screen, TEXT_COLOR };
use crate::plugins::menu::{ MenuButtonAction, menu_action, button_system };

pub fn win_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Win), win_setup)
        .add_systems(OnExit(GameState::Win), despawn_screen::<OnWinScreen>)
        .add_systems(Update, (menu_action, button_system).run_if(in_state(GameState::Win)));
}

#[derive(Component)]
struct OnWinScreen;

fn win_setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
) {
    //println!("Setting up win screen");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(css::GREEN.into()),
            OnWinScreen,
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
                    // Display the win message
                    parent.spawn((
                        Text::new("You Win!"),
                        TextFont {
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR.into()),
                    ));
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
                            MenuButtonAction::SelectLevel, // Placeholder for future actions
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Levels"),
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
                            MenuButtonAction::Quit, // Placeholder for future actions
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Quit"),
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