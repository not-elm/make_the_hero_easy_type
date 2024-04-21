use std::time::Duration;

use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{Commands, Entity, In, KeyCode, Query, Res, ResMut, With};
use bevy_flurx::action::{delay, OmitInput, once, wait};
use bevy_flurx::prelude::{ActionSeed, Pipe, Then};

use crate::consts::{GAME_CLEAR_COUNT, TWEEN_SHOW_TEXT};
use crate::plugin::stage::CorrectAnswerNum;
use crate::plugin::stage_clear::{PlayAnswerMode, RequestStageClear};
use crate::plugin::stage_ui::{RequestRegenerateStage, StageClearText};
use crate::wait_tween_event;

pub fn stage_clear() -> ActionSeed {
    once::run(update_answers)
        .pipe(play_stage_clear_se_if_release_mode())
        .then(delay::time().with(Duration::from_millis(300)))
        .then(once::event::send().with(RequestStageClear))
        .then(wait_tween_event(TWEEN_SHOW_TEXT))
        .then(wait::either(
            wait::event::comes::<RequestRegenerateStage>(),
            wait::input::just_pressed().with(KeyCode::KeyG)
        ))
        .then(once::run(reset_answer_num_if_game_cleared))
        .then(once::run(despawn_stage_clear_text))
        .omit_input()
}

fn update_answers(
    mut commands: Commands,
    mut num: ResMut<CorrectAnswerNum>,
    play_answer_mode: Option<Res<PlayAnswerMode>>,
) -> &'static str {
    if play_answer_mode.is_some() {
        num.0 += 0;
        commands.remove_resource::<PlayAnswerMode>();
        "audio/stage_clear.ogg"
    } else {
        num.0 += 1;
        if num.0 == GAME_CLEAR_COUNT {
            "audio/game_clear.ogg"
        } else {
            "audio/stage_clear.ogg"
        }
    }
}

fn play_stage_clear_se_if_release_mode() -> ActionSeed<&'static str> {
   #[cfg(not(debug_assertions))]
    {
        use bevy::audio::PlaybackSettings;
        use bevy_flurx::prelude::OmitOutput;

        once::run(|In(path): In<&'static str>| (path, PlaybackSettings::ONCE))
            .pipe(once::audio::play())
            .omit_output()
    }
    #[cfg(debug_assertions)]
    once::run(|In(_): In<&'static str>| {})
}

fn reset_answer_num_if_game_cleared(
    mut answers: ResMut<CorrectAnswerNum>,
) {
    answers.0 %= GAME_CLEAR_COUNT;
}

fn despawn_stage_clear_text(
    mut commands: Commands,
    texts: Query<Entity, With<StageClearText>>,
) {
    for entity in texts.iter() {
        commands.entity(entity).despawn_recursive();
    }
}