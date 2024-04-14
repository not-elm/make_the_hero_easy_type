use bevy::asset::Assets;
use bevy::core::Name;
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Color, ColorMaterial, Commands, default, Entity, EventWriter, Mesh, Or, Query, RegularPolygon, Res, ResMut, Text, Text2dBundle, TextStyle, Transform, With};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_flurx::action::{delay, once};
use bevy_flurx::prelude::{ActionSeed, Then};
use bevy_mod_picking::events::{Down, Pointer};
use bevy_mod_picking::PickableBundle;
use bevy_mod_picking::prelude::{ListenerInput, On};

use puzzle_core::answer::{AnswerInfo, generate_random_ratios};
use puzzle_core::calculator::small_size::SmallSizeCalculator;
use puzzle_core::stage::Stage;

use crate::arrow::remove_arrows;
use crate::consts::{CELL_COLOR, PUZZLE_HALF, PUZZLE_MARGIN};
use crate::plugin::stage::{Answer, AnswerSteps, CellNo, CellPanel, CellRatio, CellSelected, Moved, PuzzleStage, StageRatios};

pub fn setup_cells() -> ActionSeed {
    once::run(setup_stage)
        .then(once::run(spawn_cells))
}

pub fn reset_stage() -> ActionSeed {
    once::run(remove_arrows)
        .then(once::run(despawn_cells))
        .then(delay::frames().with(1))
        .then(setup_cells())
}

pub fn regenerate_stage() -> ActionSeed {
    once::run(generate_ratios)
        .then(reset_stage())
}

fn generate_ratios(
    mut commands: Commands
) {
    let ratios = generate_random_ratios::<4>();
    let answer = AnswerInfo::generate::<4, SmallSizeCalculator>(ratios);
    commands.insert_resource(StageRatios(ratios));
    commands.insert_resource(Answer(answer.ratio));
    commands.insert_resource(AnswerSteps(answer.steps));
}

fn setup_stage(
    mut commands: Commands,
    ratios: Res<StageRatios>,
) {
    commands.insert_resource(PuzzleStage(Stage::from(ratios.0)));
}

fn despawn_cells(
    mut commands: Commands,
    cells: Query<Entity, Or<(With<CellNo>, With<CellPanel>)>>,
) {
    for entity in cells.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    stage: Res<PuzzleStage>,
) {
    const LEN: f32 = PUZZLE_HALF + PUZZLE_MARGIN;

    for (i, (pos, ratio)) in [
        Vec2::new(-LEN, 0.),
        Vec2::new(0., LEN),
        Vec2::new(0., -LEN),
        Vec2::new(LEN, 0.),
    ]
        .iter()
        .zip(stage.0.ratios())
        .enumerate()
    {
        let pos = pos.extend(0.);

        commands.spawn((
            CellPanel,
            MaterialMesh2dBundle {
                mesh: meshes.add(RegularPolygon::new(PUZZLE_HALF, 4)).into(),
                material: materials.add(Color::rgb(0.15, 0.15, 0.15)),
                transform: Transform::from_translation(pos + Vec3::NEG_Z * 10.),
                ..default()
            }
        ));

        commands.spawn((
            PickableBundle::default(),
            MaterialMesh2dBundle {
                mesh: meshes.add(RegularPolygon::new(PUZZLE_HALF, 4)).into(),
                material: materials.add(CELL_COLOR),
                transform: Transform::from_translation(pos),
                ..default()
            },
            CellNo(i),
            CellRatio(ratio),
            Moved(false),
            On::<Pointer<Down>>::run(send_cell_selected),
            Name::new(format!("Cell{}", i + 1)),
        )).with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(ratio.unwrap().to_string(), TextStyle {
                    color: Color::BLACK,
                    font_size: 40.,
                    ..default()
                }),
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            });
        });
    }
}

fn send_cell_selected(
    mut ew: EventWriter<CellSelected>,
    listener: Res<ListenerInput<Pointer<Down>>>,
    cells: Query<(&CellNo, &CellRatio, &Moved)>,
) {
    if let Ok((no, ratio, moved)) = cells.get(listener.target) {
        if ratio.0.is_some() && !moved.0 {
            ew.send(CellSelected(listener.target, no.0));
        }
    }
}