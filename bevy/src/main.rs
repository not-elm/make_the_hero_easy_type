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
use crate::plugin::stage::{CellSelected, CorrectAnswerNum};
use crate::plugin::stage_clear::{InOperation, LastOne};

mod arrow;
mod plugin;
mod action;
mod consts;

fn main() {
    let mut app = App::new();
    #[cfg(target_arch = "wasm32")]
    {
        use bevy::asset::AssetMetaCheck;
        use bevy::prelude::Msaa;
        
        app
            .insert_resource(Msaa::Off)
            .insert_resource(AssetMetaCheck::Never);
    }
    app
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
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
                        update_cells(),                                     // 0: move cells
                        wait::event::comes::<LastOne>(),  // 1: stage clear
                        wait::input::just_pressed().with(KeyCode::KeyR),    // 2: retry this stage
                        wait::input::just_pressed().with(KeyCode::KeyG),    // 3: generate another stage
                        wait::input::just_pressed().with(KeyCode::KeyP),    // 4: play answer
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
                _ => {}
            }
        }
    }));
}

fn reset_answers(
    mut answers: ResMut<CorrectAnswerNum>
) {
    answers.0 = 0;
}

fn setup_stage() -> ActionSeed {
    #[cfg(feature = "release")]
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
    #[cfg(not(feature = "release"))]
    {
        regenerate_stage()
    }
}

fn update_cells() -> ActionSeed {
    select_cell()
        .then(wait::either(
            move_cell(),
            wait::event::read::<CellSelected>(), // cancel move
        ))
        .omit()
}

fn wait_tween_event(user_data: u64) -> ActionSeed {
    wait::until(move |mut er: EventReader<TweenCompleted>| er.read().any(|e| e.user_data == user_data))
}
