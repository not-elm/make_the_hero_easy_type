use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec3;
use bevy::prelude::{BackgroundColor, ButtonBundle, Color, Commands, Component, Condition, default, Display, Entity, Event, EventReader, EventWriter, IntoSystemConfigs, JustifyText, NodeBundle, Query, Res, Resource, TextBundle, TextSection, Transform, Update, With};
use bevy::text::{Text, TextStyle};
use bevy::ui::{AlignItems, FlexDirection, Interaction, JustifyContent, Style, UiRect, Val};
use bevy_flurx::prelude::switch_turned_on;
use bevy_tweening::{Animator, EaseFunction, Tween, TweenCompleted};
use bevy_tweening::lens::TransformScaleLens;

use crate::consts::{ACCENT_COLOR, GAME_CLEAR_COUNT, TWEEN_SHOW_TEXT};
use crate::plugin::secret::SecretStopWatch;
use crate::plugin::stage::{Answer, CellRatio, CorrectAnswerNum};
use crate::plugin::stage_ui::{RequestRegenerateStage, StageClearText};

#[derive(Resource, Copy, Clone, Debug, Eq, PartialEq)]
pub struct PlayAnswerMode;

/// This event is sent when there is only one cell in the stage,
/// and its number is the same as `goal`.
#[derive(Event, Copy, Clone, Debug, Eq, PartialEq)]
pub struct LastOne;

/// This structure is used as the type of [`Switch`](bevy_flurx::prelude::Switch),
/// while the switch is on, [`send_last_one`] system run.
pub struct InOperation;

/// This event is sent by [`crate::action::stage_clear`].
///
/// When it received this, update [`CorrectAnswerNum`] and start displaying the stage-clear-text animation.
#[derive(Event, Copy, Clone, Debug, Eq, PartialEq)]
pub struct RequestStageClear;

#[derive(Component)]
struct NextStageButton;

pub struct StageClearPlugin;

impl Plugin for StageClearPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LastOne>()
            .add_event::<RequestStageClear>()
            .add_systems(Update, send_last_one.run_if(switch_turned_on::<InOperation>))
            .add_systems(Update, (
                start_stage_clear_animation.run_if(come_request_stage_clear.and_then(switch_turned_on::<PlayAnswerMode>)),
                wait_animation_then_show_generate_button,
                next_stage_button_input
            ));
    }
}

fn come_request_stage_clear(
    mut er: EventReader<RequestStageClear>
) -> bool {
    let come = !er.is_empty();
    er.clear();
    come
}

fn send_last_one(
    mut ew: EventWriter<LastOne>,
    answer: Res<Answer>,
    cell: Query<&CellRatio>,
) {
    let cells = cell.iter()
        .filter_map(|ratio| ratio.0)
        .collect::<Vec<_>>();

    if cells.len() != 1 {
        return;
    }

    if cells[0] == answer.0 {
        ew.send(LastOne);
    }
}

fn start_stage_clear_animation(
    mut commands: Commands,
    answers: Res<CorrectAnswerNum>,
    stop_watch: Res<SecretStopWatch>,
) {
    commands.spawn((
        StageClearText,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Percent(8.),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK.with_a(0.7)),
            ..default()
        }
    ))
        .with_children(|parent| {
            let tween = Tween::new(
                EaseFunction::SineInOut,
                Duration::from_millis(500),
                TransformScaleLens {
                    start: Vec3::splat(0.),
                    end: Vec3::splat(1.),
                },
            )
                .with_completed_event(TWEEN_SHOW_TEXT);

            let message = if answers.0 == GAME_CLEAR_COUNT {
                secret_message(stop_watch.as_format_text())
            } else {
                stage_clear_message()
            };
            parent.spawn((
                Animator::new(tween),
                message,
            ));
        });
}

fn wait_animation_then_show_generate_button(
    mut commands: Commands,
    mut er: EventReader<TweenCompleted>,
    text_root: Query<Entity, With<StageClearText>>,
) {
    if !er.read().any(|e| e.user_data == TWEEN_SHOW_TEXT) { return; }
    let Some(root_entity) = text_root.iter().next() else {
        return;
    };
    commands.entity(root_entity).with_children(|ui| {
        ui.spawn((
            NextStageButton,
            ButtonBundle {
                style: Style{
                    padding: UiRect::all(Val::Px(8.)),
                    ..default()
                },
                background_color: BackgroundColor(ACCENT_COLOR),
                ..default()
            }
        ))
            .with_children(|ui| {
                ui.spawn(TextBundle {
                    text: Text::from_section("[G]: Generate next stage", TextStyle {
                        font_size: 64.,
                        color: Color::BLACK,
                        ..default()
                    }),
                    ..default()
                });
            });
    });
}

fn next_stage_button_input(
    mut ew: EventWriter<RequestRegenerateStage>,
    button: Query<&Interaction, With<NextStageButton>>,
) {
    for interaction in button.iter() {
        if matches!(interaction, Interaction::Pressed) {
            ew.send(RequestRegenerateStage);
        }
    }
}

fn stage_clear_message() -> TextBundle {
    TextBundle {
        text: Text::from_sections([
            TextSection::new("Stage Clear\n\n", cleat_message_style()),
        ]).with_justify(JustifyText::Center),
        transform: Transform::from_scale(Vec3::ZERO),
        ..default()
    }
}

fn secret_message(time: String) -> TextBundle {
    TextBundle {
        text: Text::from_sections([
            TextSection::new("Game Clear\n\n", cleat_message_style()),
            TextSection::new("You're the hero!\n", message_style()),
            TextSection::new(format!("Time: {time}\n"), TextStyle {
                font_size: 64.,
                color: Color::GOLD,
                ..default()
            }),
        ])
            .with_justify(JustifyText::Center),
        transform: Transform::from_scale(Vec3::ZERO),
        ..default()
    }
}

fn cleat_message_style() -> TextStyle {
    TextStyle {
        font_size: 120.,
        color: Color::GOLD,
        ..default()
    }
}

fn message_style() -> TextStyle {
    TextStyle {
        font_size: 64.,
        color: Color::GOLD,
        ..default()
    }
}

