use std::time::Duration;

use bevy::prelude::{Commands, Entity, EventWriter, In, Query, With, Without};
use bevy_flurx::prelude::*;
use puzzle_core::calculator::Calculator;
use puzzle_core::calculator::small_size::SmallSizeCalculator;
use puzzle_core::move_dir::MoveDir;

use crate::arrow::{ArrowSelected, remove_arrows};
use crate::consts::{TWEEN_SWAP_DIST, TWEEN_SWAP_SRC};
use crate::plugin::move_cell::{CombineCompleted, RequestMove};
use crate::plugin::stage::{CellNo, MoveSource};
use crate::wait_tween_event;

pub fn move_cell() -> ActionSeed {
    wait::event::read::<ArrowSelected>()
        .map(|ArrowSelected(entity)| entity)
        .through(once::run(remove_arrows))
        .through(play_move_se_if_release_mode())
        .through(delay::time().with(Duration::from_millis(100)))
        .pipe(once::run(request_move))
        .then(wait_move())
}

pub fn play_move_se_if_release_mode() -> ActionSeed {
    #[cfg(feature = "release")]
    {
        use bevy::audio::PlaybackSettings;
        once::audio::play().with(("audio/move_cell.ogg", PlaybackSettings::ONCE))
            .omit()
    }
    #[cfg(not(feature = "release"))]
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
) {
    let (src, src_no) = cell.single();
    for (dist, no) in others.iter() {
        if no.0 == SmallSizeCalculator::dist_no::<4>(src_no.0, &dir).unwrap() {
            ew.send(RequestMove {
                src,
                dist,
                dir,
            });
            return;
        }
    }
}

pub fn remove_move_source(
    mut commands: Commands,
    cells: Query<Entity, With<MoveSource>>,
) {
    for entity in cells.iter() {
        commands.entity(entity).remove::<MoveSource>();
    }
}