use bevy::app::{App, Plugin, PreUpdate, Update};
use bevy::asset::{Handle};
use bevy::core::Name;
use bevy::prelude::{Assets, Color, ColorMaterial, Component, Deref, DerefMut, Entity, Event, Parent, Query, Reflect, ReflectComponent, Res, ResMut, Resource, Visibility};
use bevy::text::Text;
use puzzle_core::answer::steps::Steps;
use puzzle_core::calculator::small_size::SmallSizeCalculator;
use puzzle_core::move_dir::MoveDir;


use puzzle_core::ratio::Ratio;
use puzzle_core::stage::RatioArray;

use crate::consts::CELL_COLOR;

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct PuzzleStage(pub puzzle_core::stage::Stage<4, SmallSizeCalculator>);

#[derive(Component)]
pub struct MoveSource;

#[derive(Component)]
pub struct MoveDist(pub MoveDir);

#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct CellPanel;

#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct CellNo(pub usize);

#[derive(Event, Debug, Copy, Clone, Eq, PartialEq, Reflect)]
pub struct CellSelected(pub Entity, pub usize);

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct CellRatio(pub Option<Ratio>);

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Moved(pub bool);

#[derive(Resource, Debug, Copy, Clone, Eq, PartialEq, Reflect)]
pub struct StageRatios(pub RatioArray<4>);

#[derive(Resource, Debug, Copy, Clone, Eq, PartialEq, Reflect, Default)]
pub struct Answer(pub Ratio);

#[derive(Resource, Debug, Clone, Eq, PartialEq)]
pub struct AnswerSteps(pub Steps);

#[derive(Resource, Debug, Clone, Eq, PartialEq, Default)]
pub struct CorrectAnswerNum(pub u64);

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CellSelected>()
            .register_type::<CellNo>()
            .register_type::<Moved>()
            .register_type::<CellSelected>()
            .register_type::<CellRatio>()
            .register_type::<CellPanel>()
            .init_resource::<PuzzleStage>()
            .init_resource::<Answer>()
            .insert_resource(CorrectAnswerNum(0))
            .add_systems(PreUpdate, update_cell_status)
            .add_systems(Update, (
                update_cell_texts,
                update_cell_colors,
            ));
    }
}

fn update_cell_status(
    mut cells: Query<(&CellNo, &mut CellRatio, &mut Moved, &mut Name)>,
    stage: Res<PuzzleStage>,
) {
    let ratios = stage.movable_ratios();
    for (no, mut ratio, mut moved, mut name) in cells.iter_mut() {
        ratio.0 = ratios[no.0].map(|r| r.ratio);
        moved.0 = ratios[no.0].map(|r| r.moved).unwrap_or_default();
        name.set(format!("Cell{}", no.0));
    }
}

fn update_cell_texts(
    cell: Query<&CellRatio>,
    mut cell_text: Query<(&Parent, &mut Text)>,
) {
    for (parent, mut text) in cell_text.iter_mut() {
        if let Ok(ratio) = cell.get(parent.get()) {
            text.sections[0].value = ratio.0.map(|r| r.to_string()).unwrap_or_default();
        }
    }
}

fn update_cell_colors(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cells: Query<(&mut Visibility, &Handle<ColorMaterial>, &CellRatio, &Moved)>,
) {
    for (mut visible, handle, ratio, moved) in cells.iter_mut() {
        if let Some(material) = materials.get_mut(handle.id()) {
            material.color = if ratio.0.is_some() && !moved.0 {
                *visible = Visibility::Visible;
                CELL_COLOR
            } else if ratio.0.is_some() && moved.0 {
                *visible = Visibility::Visible;
                Color::rgb(0.7, 0.7, 0.0)
            } else {
                *visible = Visibility::Hidden;
                CELL_COLOR.with_a(0.)
            }
        }
    }
}
