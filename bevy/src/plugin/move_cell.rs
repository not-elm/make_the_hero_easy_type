use std::time::Duration;

use bevy::app::App;
use bevy::prelude::{Commands, Component, Entity, Event, EventReader, EventWriter, Plugin, Query, ResMut, Transform, Update, Vec3, With};
use bevy_tweening::{Animator, EaseMethod, Tween, TweenCompleted};
use bevy_tweening::lens::TransformPositionLens;
use puzzle_core::move_dir::MoveDir;


use crate::consts::{TWEEN_COMBINE_SRC, TWEEN_SWAP_DIST, TWEEN_SWAP_SRC};
use crate::plugin::stage::{CellNo, Moved, PuzzleStage};

#[derive(Event, Copy, Clone, Debug, Eq, PartialEq)]
pub struct RequestMove {
    pub src: Entity,
    pub dist: Entity,
    pub dir: MoveDir,
}

#[derive(Event, Copy, Clone, Debug, Eq, PartialEq)]
pub struct StartSwap {
    pub src: Entity,
    pub dist: Entity,
}

#[derive(Event, Copy, Clone, Debug, Eq, PartialEq)]
pub struct StartCombine {
    pub src: Entity,
    pub dist: Entity,
}

#[derive(Event, Eq, PartialEq, Copy, Clone, Debug)]
pub struct CombineCompleted;

#[derive(Component, Eq, PartialEq, Copy, Clone, Debug)]
pub struct MoveSrc;

#[derive(Component, Eq, PartialEq, Copy, Clone, Debug)]
pub struct MoveDist;

pub struct MoveCellPlugin;

impl Plugin for MoveCellPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RequestMove>()
            .add_event::<StartSwap>()
            .add_event::<StartCombine>()
            .add_event::<CombineCompleted>()
            .add_systems(Update, (
                request_move,
                start_swap,
                start_combine,
                update_move_dist
            ));
    }
}

fn request_move(
    mut er: EventReader<RequestMove>,
    mut swap: EventWriter<StartSwap>,
    mut combine: EventWriter<StartCombine>,
    mut stage: ResMut<PuzzleStage>,
    cells: Query<&CellNo>,
) {
    for RequestMove { src, dist, dir } in er.read().copied() {
        stage.move_cell(cells.get(src).unwrap().0, dir);
        match dir {
            MoveDir::Up |
            MoveDir::Down |
            MoveDir::Left |
            MoveDir::Right
            => {
                swap.send(StartSwap {
                    src,
                    dist,
                });
            }
            _ => {
                combine.send(StartCombine {
                    src,
                    dist,
                });
            }
        }
    }
}

const MOVE_DURATION: u64 = 200;

fn start_swap(
    mut commands: Commands,
    mut er: EventReader<StartSwap>,
    cells: Query<(&CellNo, &Transform, &Moved)>,
) {
    for StartSwap { src, dist } in er.read().copied() {
        let Ok((src_no, src_transform, src_moved)) = cells.get(src) else {
            continue;
        };
        let Ok((dist_no, dist_transform, dist_moved)) = cells.get(dist) else {
            continue;
        };

        commands.entity(src).insert((
            *dist_no,
            *dist_moved,
            Animator::new(Tween::new(
                EaseMethod::Linear,
                Duration::from_millis(MOVE_DURATION),
                TransformPositionLens {
                    start: src_transform.translation + Vec3::Z,
                    end: dist_transform.translation + Vec3::Z,
                },
            )
                .then(Tween::new(
                    EaseMethod::Linear,
                    Duration::from_nanos(1),
                    TransformPositionLens {
                        start: dist_transform.translation,
                        end: dist_transform.translation,
                    },
                ).with_completed_event(TWEEN_SWAP_SRC)))
        ));

        commands.entity(dist)
            .insert((
                *src_no,
                *src_moved,
                Animator::new(Tween::new(
                    EaseMethod::Linear,
                    Duration::from_millis(MOVE_DURATION),
                    TransformPositionLens {
                        start: dist_transform.translation,
                        end: src_transform.translation,
                    },
                ).with_completed_event(TWEEN_SWAP_DIST))
            ));
    }
}

fn start_combine(
    mut commands: Commands,
    mut er: EventReader<StartCombine>,
    mut cells: Query<(&CellNo, &mut Transform)>,
) {
    for StartCombine { src, dist } in er.read().copied() {
        let Ok(src_component) = cells.get(src) else {
            continue;
        };
        let (src_no, src_transform) = (*src_component.0, *src_component.1);
        let Ok((dist_no, mut dist_transform)) = cells.get_mut(dist) else {
            continue;
        };

        commands.entity(src).insert((
            *dist_no,
            Animator::new(Tween::new(
                EaseMethod::Linear,
                Duration::from_millis(MOVE_DURATION),
                TransformPositionLens {
                    start: src_transform.translation + Vec3::Z,
                    end: dist_transform.translation + Vec3::Z,
                },
            )
                .with_completed_event(TWEEN_COMBINE_SRC))
        ));
        dist_transform.translation = src_transform.translation;
        commands.entity(dist).insert((
            src_no,
            MoveDist
        ));
    }
}

fn update_move_dist(
    mut ew: EventWriter<CombineCompleted>,
    mut er: EventReader<TweenCompleted>,
    mut commands: Commands,
    mut cells: Query<(Entity, &mut Transform), With<MoveDist>>,
) {
    if !er.read().any(|e| e.user_data == TWEEN_COMBINE_SRC) {
        return;
    }
    er.clear();
    for (entity, mut transform) in cells.iter_mut() {
        transform.translation += Vec3::NEG_Z;
        commands.entity(entity).remove::<MoveDist>();
    }
    ew.send(CombineCompleted);
}
