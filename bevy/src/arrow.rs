use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{Commands, Component, Entity, Event, Query, With};
use puzzle_core::move_dir::MoveDir;

#[repr(transparent)]
#[derive(Component)]
pub struct Arrow(pub MoveDir);

#[derive(Event, Clone)]
pub struct ArrowSelected(pub MoveDir);

pub fn remove_arrows(
    mut commands: Commands,
    arrows: Query<Entity, With<Arrow>>,
) {
    for e in arrows.iter() {
        commands.entity(e).despawn_recursive();
    }
}