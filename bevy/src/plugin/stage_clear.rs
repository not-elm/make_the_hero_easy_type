use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec3;
use bevy::prelude::{BackgroundColor, Color, Commands, default, Display, Event, EventReader, EventWriter, IntoSystemConfigs, JustifyText, NodeBundle, Query, Res, Resource, TextBundle, TextSection, Transform, Update};
use bevy::text::{Text, TextStyle};
use bevy::ui::{AlignItems, JustifyContent, Style, Val};
use bevy_flurx::prelude::switch_turned_on;
use bevy_tweening::{Animator, EaseFunction, Tween};
use bevy_tweening::lens::TransformScaleLens;

use crate::consts::{GAME_CLEAR_COUNT, TWEEN_SHOW_TEXT};
use crate::plugin::secret::SecretStopWatch;
use crate::plugin::stage::{Answer, CellRatio, CorrectAnswerNum};
use crate::plugin::stage_ui::StageClearText;

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

pub struct StageClearPlugin;

impl Plugin for StageClearPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LastOne>()
            .add_event::<RequestStageClear>()
            .add_systems(Update, send_last_one.run_if(switch_turned_on::<InOperation>))
            .add_systems(Update, start_stage_clear_animation.run_if(come_request_stage_clear));
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
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
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

fn stage_clear_message() -> TextBundle {
    TextBundle {
        text: Text::from_sections([
            TextSection::new("Stage Clear\n\n", cleat_message_style()),
            generate_next_stage_message(),
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
            generate_next_stage_message(),
        ])
            .with_justify(JustifyText::Center),
        transform: Transform::from_scale(Vec3::ZERO),
        ..default()
    }
}

fn generate_next_stage_message() -> TextSection {
    TextSection::new("[G]: Generate next stage", message_style())
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

