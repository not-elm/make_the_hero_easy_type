use std::f32::consts::PI;
use std::time::Duration;

use bevy::asset::AssetServer;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Color, Commands, default, EventWriter, In, IntoSystem, Query, Res, Sprite, SpriteBundle, System, Transform};
use bevy_flurx::action::{Action, delay};
use bevy_flurx::prelude::*;
use bevy_mod_picking::events::{Down, Pointer};
use bevy_mod_picking::PickableBundle;
use bevy_mod_picking::prelude::{ListenerInput, On};
use bevy_tweening::{Animator, EaseMethod, Tween, TweenCompleted};
use bevy_tweening::lens::TransformPositionLens;

use puzzle_core::move_dir::MoveDir;

use crate::arrow::{Arrow, ArrowSelected};
use crate::consts::PUZZLE_HALF;
use crate::plugin::stage::{CellSelected, MoveSource, PuzzleStage};

pub fn select_cell() -> Action<Duration> {
    delay::time().with(Duration::from_millis(100))
        .then(wait::event::read::<CellSelected>())
        .pipe(once::run(spawn_arrow(MoveDir::Right)))
        .pipe(once::run(spawn_arrow(MoveDir::RightUp)))
        .pipe(once::run(spawn_arrow(MoveDir::Up)))
        .pipe(once::run(spawn_arrow(MoveDir::LeftUp)))
        .pipe(once::run(spawn_arrow(MoveDir::Left)))
        .pipe(once::run(spawn_arrow(MoveDir::LeftDown)))
        .pipe(once::run(spawn_arrow(MoveDir::Down)))
        .pipe(once::run(spawn_arrow(MoveDir::RightDown)))
        .pipe(once::run(|In(CellSelected(cell, _)): In<CellSelected>, mut commands: Commands| {
            commands.entity(cell).insert(MoveSource);
        }))
        .then(wait::event::comes::<TweenCompleted>())
}

fn spawn_arrow(dir: MoveDir) -> impl System<In=CellSelected, Out=CellSelected> {
    IntoSystem::into_system(move |In(event): In<CellSelected>,
                                  mut commands: Commands,
                                  stage: Res<PuzzleStage>,
                                  puzzle: Query<&Transform>,
                                  asset: Res<AssetServer>,
    | {
        if !stage.0.can_move(event.1, dir) {
            return event;
        }
        let start = puzzle.get(event.0).unwrap().translation;
        #[cfg(not(debug_assertions))]
        const ASSET_PATH: &str = "arrow_release.png";
        #[cfg(debug_assertions)]
        const ASSET_PATH: &str = "arrow.png";

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(PUZZLE_HALF)),
                    color: Color::default().with_a(0.8),
                    ..default()
                },
                texture: asset.load(ASSET_PATH),
                transform: Transform::from_rotation(to_quat(&dir)),
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
            PickableBundle::default(),
            On::<Pointer<Down>>::run(send_arrow_selected),
            Arrow(dir)
        ));
        event
    })
}

fn to_quat(dir: &MoveDir) -> Quat {
    match dir {
        MoveDir::Right => Quat::default(),
        MoveDir::RightUp => Quat::from_rotation_z(PI / 4.),
        MoveDir::Up => Quat::from_rotation_z(PI / 2.),
        MoveDir::LeftUp => Quat::from_rotation_z(PI * 0.75),
        MoveDir::Left => Quat::from_rotation_z(PI),
        MoveDir::LeftDown => Quat::from_rotation_z(1.25 * PI),
        MoveDir::Down => Quat::from_rotation_z(1.5 * PI),
        MoveDir::RightDown => Quat::from_rotation_z(1.75 * PI)
    }
}

fn send_arrow_selected(
    mut ew: EventWriter<ArrowSelected>,
    input: Res<ListenerInput<Pointer<Down>>>,
    arrows: Query<&Arrow>,
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
