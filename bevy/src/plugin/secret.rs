use std::time::Duration;

use bevy::app::{App, Plugin, PostStartup, Startup};
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Color, Commands, Component, Deref, Entity, Event, EventReader, IntoSystemConfigs, KeyCode, Query, Res, ResMut, Resource, resource_exists_and_changed, TextBundle, TextStyle, Update, Visibility, With};
use bevy::text::{Text, TextSection};
use bevy::time::{Stopwatch, Time};
use bevy::utils::default;
use bevy::utils::petgraph::matrix_graph::Zero;
use bevy_flurx::prelude::switch_turned_on;
use bevy_input_sequence::{AddInputSequenceEvent, KeySequence};

use crate::plugin::stage::CorrectAnswerNum;
use crate::plugin::stage_clear::InOperation;
use crate::plugin::stage_ui::RightPanel;

#[derive(Resource, Debug, Deref)]
pub struct SecretStopWatch(Stopwatch);

impl SecretStopWatch {
    pub fn as_format_text(&self) -> String {
        let elapsed = self.0.elapsed_secs_f64();
        let hour = (elapsed / 60. / 60.) as u64;
        let minutes = (elapsed / 60. % 60.) as u64;
        let secs = (elapsed % 60.) as u64;

        format!("{hour:02}:{minutes:02}:{secs:02}")
    }
}

#[derive(Component)]
struct TimeText;

#[derive(Event, Clone)]
struct ToggleVisibility;

pub struct SecretPlugin;

impl Plugin for SecretPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_key_sequence_event::<ToggleVisibility>()
            .insert_resource(SecretStopWatch(Stopwatch::new()))
            .add_systems(Startup, setup_secret_sequence)
            .add_systems(PostStartup, setup)
            .add_systems(Update, (
                update_time_text.run_if(switch_turned_on::<InOperation>),
                toggle_visibility,
                reset_stop_watch.run_if(resource_exists_and_changed::<CorrectAnswerNum>)
            ));
    }
}

fn setup(
    mut commands: Commands,
    right_panel: Query<Entity, With<RightPanel>>,
) {
    let id = commands.spawn((
        TimeText,
        TextBundle {
            visibility: Visibility::Hidden,
            text: Text::from_sections([
                TextSection::new("time: ", TextStyle {
                    font_size: 32.,
                    ..default()
                }),
                TextSection::new("00:00:00", TextStyle {
                    font_size: 32.,
                    color: Color::ORANGE,
                    ..default()
                }),
            ]),
            ..default()
        }
    ))
        .id();
    commands.entity(right_panel.single()).add_child(id);
}

fn setup_secret_sequence(
    mut commands: Commands
) {
    commands.spawn(KeySequence::new(
        ToggleVisibility,
        [
            KeyCode::KeyT,
            KeyCode::KeyI,
            KeyCode::KeyM,
            KeyCode::KeyE
        ],
    )
        .time_limit(Duration::from_secs(3)));
}

fn update_time_text(
    mut stopwatch: ResMut<SecretStopWatch>,
    mut text: Query<&mut Text, With<TimeText>>,
    time: Res<Time>,
) {
    stopwatch.0.tick(time.delta());
    let elapsed_text = stopwatch.as_format_text();
    for mut text in text.iter_mut() {
        text.sections[1].value = elapsed_text.clone();
    }
}

fn reset_stop_watch(
    mut stopwatch: ResMut<SecretStopWatch>,
    answers: Res<CorrectAnswerNum>,
) {
    if answers.0.is_zero() {
        stopwatch.0.reset();
    }
}

fn toggle_visibility(
    mut er: EventReader<ToggleVisibility>,
    mut time_text: Query<&mut Visibility, With<TimeText>>,
) {
    for _ in er.read() {
        for mut visibility in time_text.iter_mut() {
            *visibility = if !matches!(*visibility, Visibility::Visible) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}