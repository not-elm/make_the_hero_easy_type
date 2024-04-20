use std::time::Duration;

use bevy::prelude::{Commands, Entity, EventWriter, In, Query, ResMut, With, Without};
use bevy_flurx::prelude::*;

use puzzle_core::calculator::Calculator;
use puzzle_core::calculator::small_size::SmallSizeCalculator;
use puzzle_core::move_dir::MoveDir;

use crate::{CellAct, wait_tween_event};
use crate::arrow::{ArrowSelected, clean_up_movable_cells, remove_arrows};
use crate::consts::{TWEEN_SWAP_DIST, TWEEN_SWAP_SRC};
use crate::plugin::move_cell::{CombineCompleted, RequestMove};
use crate::plugin::stage::{CellNo, MoveSource, PuzzleStage};

pub fn move_cell() -> ActionSeed {
    wait::event::read::<ArrowSelected>()
        .map(|ArrowSelected(entity)| entity)
        .through(once::run(clean_up_movable_cells))
        .through(once::run(remove_arrows))
        .through(play_move_se_if_release_mode())
        .through(delay::time().with(Duration::from_millis(100)))
        .pipe(once::run(request_move))
        .then(push_undo())
        .then(wait_move())
}

pub fn play_move_se_if_release_mode() -> ActionSeed {
   #[cfg(not(debug_assertions))]
    {
        use bevy::audio::PlaybackSettings;
        once::audio::play().with(("audio/move_cell.ogg", PlaybackSettings::ONCE))
            .omit()
    }
    #[cfg(debug_assertions)]
    once::run(|| {})
}

pub fn wait_move() -> ActionSeed {
    wait::either(
        wait_swap(),
        wait::event::comes::<CombineCompleted>(),
    )
        .then(once::run(remove_move_source))
        .omit()
}

fn wait_swap() -> ActionSeed {
    wait::both(
        wait_tween_event(TWEEN_SWAP_SRC),
        wait_tween_event(TWEEN_SWAP_DIST),
    )
        .omit()
}

fn request_move(
    In(dir): In<MoveDir>,
    mut ew: EventWriter<RequestMove>,
    cell: Query<(Entity, &CellNo), With<MoveSource>>,
    others: Query<(Entity, &CellNo), Without<MoveSource>>,
) -> (Entity, Entity, MoveDir) {
    let (src, src_no) = cell.single();
    for (dist, no) in others.iter() {
        if no.0 == SmallSizeCalculator::dist_no::<4>(src_no.0, &dir).unwrap() {
            ew.send(RequestMove {
                src,
                dist,
                dir,
            });
            return (src, dist, dir);
        }
    }
    panic!("unreachable");
}

fn push_undo() -> ActionSeed {
    record::push().with(Track {
        act: CellAct,
        rollback: Rollback::parts(
            Undo::make(|| once::run(move |mut stage: ResMut<PuzzleStage>| {
                stage.undo();
            })),
            Redo::make(|_| once::run(move |mut stage: ResMut<PuzzleStage>| {
                stage.redo();
            })),
        ),
    })
        .omit()
}

pub fn remove_move_source(
    mut commands: Commands,
    cells: Query<Entity, With<MoveSource>>,
) {
    for entity in cells.iter() {
        commands.entity(entity).remove::<MoveSource>();
    }
}