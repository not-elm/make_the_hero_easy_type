#![allow(clippy::type_complexity)]

use bevy::app::{App, PluginGroup, Startup, Update};
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, ClearColor, Color, Commands, EventReader, KeyCode, ResMut, Window};
use bevy::utils::default;
use bevy::window::WindowPlugin;
use bevy_flurx::actions;
use bevy_flurx::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_tweening::{TweenCompleted, TweeningPlugin};

use crate::action::cell_select::select_cell;
use crate::action::cleanup::cleanup;
use crate::action::move_cell::move_cell;
use crate::action::play_answer::{exists_steps, play_next_step, setup_step_resource};
use crate::action::setup_cells::{regenerate_stage, reset_stage};
use crate::action::stage_clear::stage_clear;
use crate::arrow::ArrowSelected;
use crate::plugin::PuzzlePlugins;
use crate::plugin::stage::{CellSelected, CorrectAnswerNum, RequestCancelMove};
use crate::plugin::stage_clear::{InOperation, LastOne};
use crate::plugin::stage_ui::{RequestCellRedo, RequestCellUndo, RequestPlayAnswerMode, RequestRegenerateStage, RequestResetStage};

mod arrow;
mod plugin;
mod action;
mod consts;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }));
    #[cfg(target_arch = "wasm32")]
    {
        use bevy::asset::AssetMetaCheck;
        use bevy::prelude::Msaa;

        app
            .insert_resource(Msaa::Off)
            .insert_resource(AssetMetaCheck::Never);
    }
    #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(WorldInspectorPlugin::new());
    }
    app
        .add_plugins((
            FlurxPlugin,
            DefaultPickingPlugins,
            TweeningPlugin,
            PuzzlePlugins
        ))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_event::<ArrowSelected>()
        .add_systems(Startup, (
            spawn_camera,
            spawn_reactor
        ))
        .run();
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CellAct;

/// It has been described the sequence from the start to the end of the game in this function.
///
/// Since it is this game is simply, all the game processed are written in one [`Reactor`],
/// but for a game of certain size, it is recommended to use [`Reactor`] on a per-`Scene` basis or 
/// only in parts such as waiting for a complex animation.
fn spawn_reactor(mut commands: Commands) {
    commands.spawn(Reactor::schedule(|task| async move {
        task.will(Update, setup_stage()).await;
        loop {
            let end_action_index = task.will(Update, {
                once::switch::on::<InOperation>()
                    .then(wait::any(actions![
                        // 0: move cells
                        update_cells(),   
                        // 1: stage clear
                        wait::event::comes::<LastOne>(),   
                        // 2: retry this stage
                        request_reset_stage(),    
                        // 3: generate another stage
                        request_regenerate_stage(),   
                        // 4: play answer
                        request_play_answer_mode(),
                        // 5: undo
                        request_undo(),    
                        // 6: redo
                        request_redo(),    
                    ]))
                    .through(cleanup())
            }).await;

            match end_action_index {
                1 => {
                    task.will(Update, stage_clear().then(regenerate_stage())).await;
                }
                2 => {
                    task.will(Update, reset_stage()).await;
                }
                3 => {
                    task.will(Update, once::run(reset_answers)
                        .then(regenerate_stage()),
                    ).await;
                }
                4 => {
                    task.will(Update, setup_step_resource()).await;
                    while task.will(Update, exists_steps()).await {
                        task.will(Update, play_next_step()).await;
                    }
                }
                5 => {
                    let _ = task.will(Update, record::undo::once::<CellAct>()).await;
                }
                6 => {
                    let _ = task.will(Update, record::redo::once::<CellAct>()).await;
                }
                _ => {}
            }
        }
    }));
}

fn request_reset_stage() -> ActionSeed {
    wait::either(
        wait::input::just_pressed().with(KeyCode::KeyR),
        wait::event::comes::<RequestResetStage>(),
    )
        .omit()
}

fn request_regenerate_stage() -> ActionSeed {
    wait::either(
        wait::input::just_pressed().with(KeyCode::KeyG),
        wait::event::comes::<RequestRegenerateStage>(),
    )
        .omit()
}

fn request_play_answer_mode() -> ActionSeed {
    wait::either(
        wait::input::just_pressed().with(KeyCode::KeyP),
        wait::event::comes::<RequestPlayAnswerMode>(),
    )
        .omit()
}

fn request_undo() -> ActionSeed {
    wait::either(
        wait::input::just_pressed().with(KeyCode::KeyZ),
        wait::event::comes::<RequestCellUndo>(),
    )
        .omit()
}

fn request_redo() -> ActionSeed {
    wait::either(
        wait::input::just_pressed().with(KeyCode::KeyX),
        wait::event::comes::<RequestCellRedo>(),
    )
        .omit()
}

fn reset_answers(
    mut answers: ResMut<CorrectAnswerNum>
) {
    answers.0 = 0;
}

fn setup_stage() -> ActionSeed {
    #[cfg(not(debug_assertions))]
    {
        use bevy::audio::{PlaybackMode, PlaybackSettings, Volume};
        once::audio::play().with(("audio/bgm.ogg", PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.1),
            ..default()
        }))
            .then(regenerate_stage())
            .omit()
    }

    #[cfg(debug_assertions)]
    {
        regenerate_stage()
    }
}

fn update_cells() -> ActionSeed {
    select_cell()
        .then(wait::any(actions![
             move_cell(),
            wait::event::read::<CellSelected>(), // cancel move
            wait::event::comes::<RequestCancelMove>()
        ]))
        .omit()
}

fn wait_tween_event(user_data: u64) -> ActionSeed {
    wait::until(move |mut er: EventReader<TweenCompleted>| er.read().any(|e| e.user_data == user_data))
}
