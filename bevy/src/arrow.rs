use bevy::prelude::{Commands, Component, Entity, Event, Query, With};
use bevy_mod_picking::events::{Down, Pointer};
use bevy_mod_picking::prelude::On;

use puzzle_core::move_dir::MoveDir;

use crate::action::setup_cells::send_cell_selected;

#[repr(transparent)]
#[derive(Component)]
pub struct MovableCell(pub MoveDir);

#[derive(Component)]
pub struct Arrow;

#[derive(Event, Clone)]
pub struct ArrowSelected(pub MoveDir);

pub fn clean_up_movable_cells(
    mut commands: Commands,
    arrows: Query<Entity, With<MovableCell>>,
) {
    for e in arrows.iter() {
        commands.entity(e).remove::<MovableCell>();
        commands
            .entity(e)
            .insert(On::<Pointer<Down>>::run(send_cell_selected));
    }
}

pub fn remove_arrows(
    mut commands: Commands,
    arrows: Query<Entity, With<Arrow>>,
) {
    for entity in arrows.iter() {
        commands.entity(entity).despawn();
    }
}