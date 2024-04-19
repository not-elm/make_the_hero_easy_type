use bevy::app::{App, Plugin, PreStartup, Startup};
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Color, Commands, Component, Entity, IntoSystemConfigs, NodeBundle, PositionType, Query, Reflect, ReflectComponent, Res, resource_exists_and_changed, Text, TextBundle, Update, Val, With};
use bevy::text::{TextSection, TextStyle};
use bevy::ui::{Display, FlexDirection, Style};
use bevy::utils::default;
use bevy_mod_picking::picking_core::Pickable;

use crate::plugin::stage::{Answer, CorrectAnswerNum};

#[derive(Copy, Clone, Component, Reflect, Debug, Eq, PartialEq)]
#[reflect(Component)]
pub struct RootUi;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Eq, PartialEq)]
#[reflect(Component)]
pub struct RightPanel;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Eq, PartialEq)]
#[reflect(Component)]
struct AnswerText;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Eq, PartialEq)]
#[reflect(Component)]
struct CorrectAnswerNumText;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Eq, PartialEq)]
#[reflect(Component)]
pub struct StageClearText;

pub struct StageUiPlugin;

impl Plugin for StageUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<RootUi>()
            .register_type::<AnswerText>()
            .register_type::<CorrectAnswerNumText>()
            .register_type::<StageClearText>()
            .register_type::<RightPanel>()
            .add_systems(PreStartup, spawn_root_ui)
            .add_systems(Startup, (
                spawn_input_label,
                spawn_right_panel
            ).chain())
            .add_systems(Update, (
                update_answer_text.run_if(resource_exists_and_changed::<Answer>),
                update_correct_answer_num_text.run_if(resource_exists_and_changed::<CorrectAnswerNum>),
            ));
    }
}

fn spawn_root_ui(
    mut commands: Commands
) {
    commands.spawn((
        RootUi,
        Pickable::IGNORE,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                ..default()
            },
            ..default()
        }
    ));
}

fn spawn_input_label(
    mut commands: Commands,
    root: Query<Entity, With<RootUi>>,
) {
    let text_style = {
        TextStyle {
            font_size: 32.,
            ..default()
        }
    };
    let input_label = commands.spawn((
        Pickable::IGNORE,
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.),
                ..default()
            },
            ..default()
        }
    ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("[R]: Reset stage", text_style.clone()),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section("[G]: Generate a new stage", text_style.clone()),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section("[P]: Play answer", text_style.clone()),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section("[Z]: Undo", text_style.clone()),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section("[X]: Redo", text_style),
                ..default()
            });
        })
        .id();

    commands
        .entity(root.single())
        .add_child(input_label);
}

fn spawn_right_panel(
    mut commands: Commands,
    root: Query<Entity, With<RootUi>>,
) {
    const LABEL_FONT_SIZE: f32 = 32.;
    let input_label = commands.spawn((
        Pickable::IGNORE,
        RightPanel,
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                row_gap: Val::Px(16.),
                right: Val::Px(16.),
                top: Val::Px(16.),
                ..default()
            },
            ..default()
        }
    ))
        .with_children(|parent| {
            parent.spawn((
                AnswerText,
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new("goal: ", TextStyle {
                            font_size: LABEL_FONT_SIZE,
                            ..default()
                        }),
                        TextSection::new("", TextStyle {
                            font_size: LABEL_FONT_SIZE,
                            color: Color::GOLD,
                            ..default()
                        }),
                    ]),
                    ..default()
                }
            ));

            parent.spawn((
                CorrectAnswerNumText,
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new("consecutive answers: ", TextStyle {
                            font_size: LABEL_FONT_SIZE,
                            ..default()
                        }),
                        TextSection::new("0", TextStyle {
                            font_size: LABEL_FONT_SIZE,
                            color: Color::GOLD,
                            ..default()
                        })
                    ]),
                    ..default()
                }
            ));
        })
        .id();

    commands
        .entity(root.single())
        .add_child(input_label);
}

fn update_answer_text(
    answer: Res<Answer>,
    mut text: Query<&mut Text, With<AnswerText>>,
) {
    for mut text in text.iter_mut() {
        text.sections[1].value = format!("{}", answer.0);
    }
}

fn update_correct_answer_num_text(
    mut text: Query<&mut Text, With<CorrectAnswerNumText>>,
    num: Res<CorrectAnswerNum>,
) {
    for mut text in text.iter_mut() {
        text.sections[1].value = format!("{}", num.0);
    }
}
