use std::time::Duration;

use bevy::hierarchy::BuildChildren;
use bevy::log::debug;
use bevy::prelude::{Color, Commands, Component, Entity, EventWriter, KeyCode, Query, Res, ResMut, Resource, TextBundle, With};
use bevy::text::{Text, TextSection, TextStyle};
use bevy::utils::default;
use bevy_flurx::action::{delay, OmitInput, once, wait};
use bevy_flurx::prelude::{ActionSeed, Either, Map, Then, Through};

use puzzle_core::answer::steps::Steps;
use puzzle_core::calculator::Calculator;
use puzzle_core::calculator::small_size::SmallSizeCalculator;

use crate::action::move_cell::{play_move_se_if_release_mode, wait_move};
use crate::action::setup_cells::reset_stage;
use crate::plugin::move_cell::RequestMove;
use crate::plugin::stage::{AnswerSteps, CellNo};
use crate::plugin::stage_clear::PlayAnswerMode;
use crate::plugin::stage_ui::RootUi;

#[derive(Resource)]
struct TmpSteps(Steps);

pub fn setup_step_resource() -> ActionSeed {
    reset_stage()
        .then(once::run(|mut commands: Commands, steps: Res<AnswerSteps>| {
            commands.insert_resource(TmpSteps(steps.0.clone()));
            commands.insert_resource(PlayAnswerMode);
        }))
}

pub fn exists_steps() -> ActionSeed<(), bool> {
    once::run(|steps: Res<TmpSteps>| !steps.0.is_empty())
}

pub fn play_next_step() -> ActionSeed {
    play_move_se_if_release_mode()
        .then(delay::time().with(Duration::from_millis(300)))
        .then(once::run(next_step_move))
        .then(wait_move())
        .then(delay::time().with(Duration::from_millis(600)))
        .omit_input()
}


#[derive(Debug)]
pub enum AnswerModeNextStatus {
    Replay,
    NextStage,
}

pub fn wait_input_next_status() -> ActionSeed<(), AnswerModeNextStatus> {
    once::run(spawn_complete_text)
        .then(wait::either(
            wait::input::just_pressed().with(KeyCode::KeyR),
            wait::input::just_pressed().with(KeyCode::KeyG),
        ))
        .through(once::run(despawn_complete_text))
        .map(|either| match either {
            Either::Left(_) => AnswerModeNextStatus::Replay,
            Either::Right(_) => AnswerModeNextStatus::NextStage
        })
}

#[derive(Component)]
struct CompleteText;

fn spawn_complete_text(
    mut commands: Commands,
    root: Query<Entity, With<RootUi>>,
) {
    commands
        .entity(root.single())
        .with_children(|ui| {
            let text_style = TextStyle {
                font_size: 32.,
                color: Color::GOLD,
                ..default()
            };
            ui.spawn((
                CompleteText,
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new("Answer mode has finished\n\n", text_style.clone()),
                        TextSection::new("[R]: Replay\n", text_style.clone()),
                        TextSection::new("[G]: Generate next stage", text_style.clone()),
                    ]),
                    ..default()
                }
            ));
        });
}

fn despawn_complete_text(
    mut commands: Commands,
    texts: Query<Entity, With<CompleteText>>,
) {
    for entity in texts.iter() {
        commands.entity(entity).despawn();
    }
}

fn next_step_move(
    mut steps: ResMut<TmpSteps>,
    mut ew: EventWriter<RequestMove>,
    cells: Query<(Entity, &CellNo)>,
) {
    let (n, dir) = steps.0.pop_front().unwrap();
    let Some(dist_no) = SmallSizeCalculator::dist_no::<4>(n, &dir) else {
        return;
    };
    debug!("no: {n} move dir: {dir:?}");

    ew.send(RequestMove {
        src: cells.iter().find_map(|(e, no)| {
            (no.0 == n).then_some(e)
        }).unwrap(),
        dist: cells.iter().find_map(|(e, no)| {
            (no.0 == dist_no).then_some(e)
        }).unwrap(),
        dir,
    });
}