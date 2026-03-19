use std::sync::mpsc::{self, Sender};

use proteus_lib::container::play_settings::EffectSettings;
use proteus_lib::container::prot::PathsTrack;
use tauri::{Manager, State, Window};

use super::types::{
    InlineEffectsResult, PlayerActorState, PlayerCommand, PlayerFlags, ShuffleSchedule,
};

fn ask<T>(state: &State<PlayerActorState>, build: impl FnOnce(Sender<T>) -> PlayerCommand) -> T
where
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    state.send(build(tx));
    rx.recv().expect("player actor thread terminated")
}

fn window_label(window: &Window) -> String {
    window.label().to_string()
}

pub fn replace_window_player(
    window: &Window,
    state: &State<PlayerActorState>,
    tracks: Vec<PathsTrack>,
    effects: Vec<EffectSettings>,
) {
    let app_handle = window.app_handle().clone();
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::Replace {
        label,
        tracks,
        effects,
        app_handle,
        reply,
    });
}

pub fn clear_window_player_by_label(label: &str, state: &State<PlayerActorState>) {
    let label = label.to_string();
    ask(state, move |reply| PlayerCommand::Clear { label, reply });
}

pub fn player_resume_state(window: &Window, state: &State<PlayerActorState>) -> (bool, f64) {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::GetResumeState {
        label,
        reply,
    })
}

pub fn player_play(
    window: &Window,
    state: &State<PlayerActorState>,
    effects: Vec<EffectSettings>,
) {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::Play {
        label,
        effects,
        reply,
    });
}

pub fn player_pause(window: &Window, state: &State<PlayerActorState>) {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::Pause { label, reply });
}

pub fn player_stop(window: &Window, state: &State<PlayerActorState>) {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::Stop { label, reply });
}

pub fn player_seek(window: &Window, state: &State<PlayerActorState>, position: f64) {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::Seek {
        label,
        position,
        reply,
    });
}

pub fn player_shuffle(window: &Window, state: &State<PlayerActorState>) -> bool {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::Shuffle { label, reply })
}

pub fn player_position(window: &Window, state: &State<PlayerActorState>) -> f64 {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::GetPosition {
        label,
        reply,
    })
}

pub fn player_duration(window: &Window, state: &State<PlayerActorState>) -> f64 {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::GetDuration {
        label,
        reply,
    })
}

pub fn player_ids(window: &Window, state: &State<PlayerActorState>) -> Option<Vec<String>> {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::GetIds { label, reply })
}

pub fn player_set_volume(window: &Window, state: &State<PlayerActorState>, volume: f32) {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::SetVolume {
        label,
        volume,
        reply,
    });
}

pub fn player_set_track_mix(
    window: &Window,
    state: &State<PlayerActorState>,
    slot_index: usize,
    level: f32,
    pan: f32,
) {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::SetTrackMix {
        label,
        slot_index,
        level,
        pan,
        reply,
    });
}

pub fn player_volume(window: &Window, state: &State<PlayerActorState>) -> f32 {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::GetVolume {
        label,
        reply,
    })
}

pub fn player_set_effects_inline(
    window: &Window,
    state: &State<PlayerActorState>,
    effects: Vec<EffectSettings>,
) -> InlineEffectsResult {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::SetEffectsInline {
        label,
        effects,
        reply,
    })
}

pub fn player_flags(window: &Window, state: &State<PlayerActorState>) -> PlayerFlags {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::GetFlags { label, reply })
}

pub fn player_levels(window: &Window, state: &State<PlayerActorState>) -> Vec<f32> {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::GetLevels {
        label,
        reply,
    })
}

pub fn player_levels_db(window: &Window, state: &State<PlayerActorState>) -> Vec<f32> {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::GetLevelsDb {
        label,
        reply,
    })
}

pub fn player_shuffle_schedule(
    window: &Window,
    state: &State<PlayerActorState>,
) -> Option<ShuffleSchedule> {
    let label = window_label(window);
    ask(state, move |reply| PlayerCommand::GetShuffleSchedule {
        label,
        reply,
    })
}
