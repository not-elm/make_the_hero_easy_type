use bevy_flurx::action::{Omit, once};
use bevy_flurx::prelude::{ActionSeed, Then};

use crate::action::move_cell::remove_move_source;
use crate::arrow::remove_arrows;
use crate::plugin::stage_clear::InOperation;

pub fn cleanup() -> ActionSeed {
    once::run(remove_arrows)
        .then(once::run(remove_move_source))
        .then(once::switch::off::<InOperation>())
        .omit()
}

