use std::time::Duration;

use bevy::log::debug;
use bevy::prelude::{Commands, Entity, EventWriter, Query, Res, ResMut, Resource};
use bevy_flurx::action::{delay, OmitInput, once};
use bevy_flurx::prelude::{ActionSeed, Then};

use puzzle_core::answer::steps::Steps;
use puzzle_core::calculator::Calculator;
use puzzle_core::calculator::small_size::SmallSizeCalculator;

use crate::action::move_cell::{play_move_se_if_release_mode, wait_move};
use crate::action::setup_cells::reset_stage;
use crate::plugin::move_cell::RequestMove;
use crate::plugin::stage::{AnswerSteps, CellNo};
use crate::plugin::stage_clear::PlayAnswerMode;

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
        .then(delay::time().with(Duration::from_millis(100)))
        .then(once::run(next_step_move))
        .then(wait_move())
        .then(delay::time().with(Duration::from_millis(300)))
        .omit_input()
}

fn next_step_move(
    mut steps: ResMut<TmpSteps>,
    mut ew: EventWriter<RequestMove>,
    cells: Query<(Entity, &CellNo)>,
) {
    let (n, dir) = steps.0.pop_front().unwrap();
    let Some(dist_no) = SmallSizeCalculator::dist_no::<4>(n, &dir) else{
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