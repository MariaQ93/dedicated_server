use super::components::*;
use crate::game::components::GameResult;
use crate::AppState;
use bevy::prelude::*;

pub fn load_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_result: Res<GameResult>,
) {
    let result: String = if game_result.id == 0 {
        "You Won!".to_string()
    } else {
        "You Lost!".to_string()
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(NBundle)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    result,
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 80.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(100.),
                    ..default()
                },
                ..default()
            });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(265.),
                        width: Val::Px(230.0),
                        height: Val::Px(90.0),
                        border: UiRect::all(Val::Px(3.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        // center the button within its parent container
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgb(0.071, 0.141, 0.753).into(),
                    ..default()
                })
                .insert(PlayAgainButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play Again",
                        TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(407.5),
                        width: Val::Px(230.0),
                        height: Val::Px(90.0),
                        border: UiRect::all(Val::Px(3.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        // center the button within its parent container
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgb(0.071, 0.141, 0.753).into(),
                    ..default()
                })
                .insert(ExitButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit To Menu",
                        TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn tear_down_screen(mut commands: Commands, mut node_query: Query<Entity, With<NBundle>>) {
    let node = node_query.single_mut();
    commands.entity(node).despawn_recursive();
}

pub fn play_again_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<PlayAgainButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
                app_state_next_state.set(AppState::ServerRunning);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.133, 0.188, 0.659).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::rgb(0.071, 0.141, 0.753).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn main_menu_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ExitButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.075, 0.118, 0.502).into();
                border_color.0 = Color::RED;
                app_state_next_state.set(AppState::GameOptions);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.133, 0.188, 0.659).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::rgb(0.071, 0.141, 0.753).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
