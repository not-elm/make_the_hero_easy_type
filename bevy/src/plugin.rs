use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugin::move_cell::MoveCellPlugin;
use crate::plugin::secret::SecretPlugin;
use crate::plugin::stage::StagePlugin;
use crate::plugin::stage_clear::StageClearPlugin;
use crate::plugin::stage_ui::StageUiPlugin;

pub mod move_cell;
pub mod stage;
pub mod stage_clear;
pub mod stage_ui;
mod secret;


pub struct PuzzlePlugins;

impl PluginGroup for PuzzlePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(StagePlugin)
            .add(StageUiPlugin)
            .add(MoveCellPlugin)
            .add(StageClearPlugin)
            .add(SecretPlugin)
            .build()
    }
}




