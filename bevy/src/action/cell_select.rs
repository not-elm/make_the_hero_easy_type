use std::time::Duration;

use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Color, Commands, Component, default, Entity, EventWriter, In, Query, Res, Sprite, SpriteBundle, Transform};
use bevy_flurx::action::{Action, delay};
use bevy_flurx::prelude::*;
use bevy_mod_picking::events::{Down, Pointer};
use bevy_mod_picking::prelude::{ListenerInput, On};
use bevy_tweening::{Animator, EaseMethod, Tween, TweenCompleted};
use bevy_tweening::lens::TransformPositionLens;

use puzzle_core::move_dir::MoveDir;

use crate::arrow::{Arrow, ArrowSelected, MovableCell};
use crate::consts::PUZZLE_HALF;
use crate::plugin::stage::{CellNo, CellSelected, MoveSource, PuzzleStage};

#[derive(Component)]
pub struct PreviewText;


pub fn select_cell() -> Action<Duration> {
    delay::time().with(Duration::from_millis(100))
        .then(wait::event::read::<CellSelected>())
        .pipe(setup_dir(MoveDir::Right))
        .pipe(setup_dir(MoveDir::RightUp))
        .pipe(setup_dir(MoveDir::Up))
        .pipe(setup_dir(MoveDir::LeftUp))
        .pipe(setup_dir(MoveDir::Left))
        .pipe(setup_dir(MoveDir::LeftDown))
        .pipe(setup_dir(MoveDir::Down))
        .pipe(setup_dir(MoveDir::RightDown))
        .pipe(once::run(|In(CellSelected(cell, _)): In<CellSelected>, mut commands: Commands| {
            commands.entity(cell).insert(MoveSource);
        }))
        .then(wait::event::comes::<TweenCompleted>())
}

fn setup_dir(dir: MoveDir) -> ActionSeed<CellSelected, CellSelected> {
    ActionSeed::define(move |event: CellSelected| {
        once::run(spawn_arrow).with((dir, event))
            .then(once::run(replace_cell_point_down_handle).with((dir, event)))
            .overwrite(event)
    })
}

fn spawn_arrow(
    In((dir, CellSelected(src_entity, src_no))): In<(MoveDir, CellSelected)>,
    mut commands: Commands,
    stage: Res<PuzzleStage>,
    puzzle: Query<&Transform>,
    asset: Res<AssetServer>,
) {
    if !stage.0.can_move(src_no, dir) {
        return;
    }
    let start = puzzle.get(src_entity).unwrap().translation;

    commands.spawn((
        Arrow,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(32.)),
                color: Color::default().with_a(0.9),
                ..default()
            },
            texture: asset.load(dir.asset_path()),
            ..default()
        },
        Animator::new(Tween::new(
            EaseMethod::Linear,
            Duration::from_millis(200),
            TransformPositionLens {
                start,
                end: to_vec3(dir) * PUZZLE_HALF + Vec3::new(0., 0., 10.) + start,
            },
        ).with_completed_event(0)),
    ));
}

fn replace_cell_point_down_handle(
    In((dir, CellSelected(_, cell_no))): In<(MoveDir, CellSelected)>,
    mut commands: Commands,
    stage: Res<PuzzleStage>,
    cells: Query<(Entity, &CellNo)>,
) {
    if !stage.can_move(cell_no, dir) {
        return;
    }

    if let Some((entity, _)) = stage
        .move_dist_no(cell_no, &dir)
        .and_then(|dist_no| cells.iter().find(|(_, no)| no.0 == dist_no)) {
        commands.entity(entity).insert((
            MovableCell(dir),
            On::<Pointer<Down>>::run(send_arrow_selected),
        ));
    }
}

fn send_arrow_selected(
    mut ew: EventWriter<ArrowSelected>,
    input: Res<ListenerInput<Pointer<Down>>>,
    arrows: Query<&MovableCell>,
) {
    ew.send(ArrowSelected(arrows.get(input.target).unwrap().0));
}

fn to_vec3(dir: MoveDir) -> Vec3 {
    const D: f32 = 0.85;
    match dir {
        MoveDir::Down => Vec3::NEG_Y * D,
        MoveDir::LeftUp => Vec3::new(-0.5, 0.5, 0.0),
        MoveDir::Up => Vec3::Y * D,
        MoveDir::RightUp => Vec3::new(0.5, 0.5, 0.0),
        MoveDir::Left => Vec3::NEG_X * D,
        MoveDir::Right => Vec3::X * D,
        MoveDir::LeftDown => Vec3::new(-0.5, -0.5, 0.0),
        MoveDir::RightDown => Vec3::new(0.5, -0.5, 0.0),
    }
}
