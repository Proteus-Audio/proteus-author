use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use proteus_lib::container::play_settings::EffectSettings;
use proteus_lib::container::prot::PathsTrack;
use proteus_lib::diagnostics::reporter::Report;
use proteus_lib::playback::player::Player;
use tauri::{AppHandle, Emitter, Manager, State, Window};

pub type ShuffleSchedule = Vec<(f64, Vec<Vec<String>>)>;

pub struct PlayerActorState {
    tx: Sender<PlayerCommand>,
}

impl PlayerActorState {
    fn send(&self, command: PlayerCommand) {
        let _ = self.tx.send(command);
    }
}

pub fn create_player_actor_state() -> PlayerActorState {
    let (tx, rx) = mpsc::channel::<PlayerCommand>();
    thread::spawn(move || player_worker(rx));
    PlayerActorState { tx }
}

#[derive(Debug, Clone, Copy)]
pub enum InlineEffectsResult {
    Applied,
    NoPlayer,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerFlags {
    pub exists: bool,
    pub is_playing: bool,
    pub is_paused: bool,
}

enum PlayerCommand {
    Replace {
        label: String,
        tracks: Vec<PathsTrack>,
        effects: Vec<EffectSettings>,
        app_handle: AppHandle,
        reply: Sender<()>,
    },
    Clear {
        label: String,
        reply: Sender<()>,
    },
    GetResumeState {
        label: String,
        reply: Sender<(bool, f64)>,
    },
    Play {
        label: String,
        effects: Vec<EffectSettings>,
        reply: Sender<()>,
    },
    Pause {
        label: String,
        reply: Sender<()>,
    },
    Stop {
        label: String,
        reply: Sender<()>,
    },
    Seek {
        label: String,
        position: f64,
        reply: Sender<()>,
    },
    Shuffle {
        label: String,
        reply: Sender<bool>,
    },
    GetPosition {
        label: String,
        reply: Sender<f64>,
    },
    GetDuration {
        label: String,
        reply: Sender<f64>,
    },
    GetIds {
        label: String,
        reply: Sender<Option<Vec<String>>>,
    },
    SetVolume {
        label: String,
        volume: f32,
        reply: Sender<()>,
    },
    SetTrackMix {
        label: String,
        slot_index: usize,
        level: f32,
        pan: f32,
        reply: Sender<()>,
    },
    GetVolume {
        label: String,
        reply: Sender<f32>,
    },
    SetEffectsInline {
        label: String,
        effects: Vec<EffectSettings>,
        reply: Sender<InlineEffectsResult>,
    },
    GetFlags {
        label: String,
        reply: Sender<PlayerFlags>,
    },
    GetLevels {
        label: String,
        reply: Sender<Vec<f32>>,
    },
    GetLevelsDb {
        label: String,
        reply: Sender<Vec<f32>>,
    },
    GetShuffleSchedule {
        label: String,
        reply: Sender<Option<ShuffleSchedule>>,
    },
}

fn player_worker(rx: Receiver<PlayerCommand>) {
    let mut players: HashMap<String, Option<Player>> = HashMap::new();

    while let Ok(command) = rx.recv() {
        match command {
            PlayerCommand::Replace {
                label,
                tracks,
                effects,
                app_handle,
                reply,
            } => {
                if tracks.is_empty() {
                    if let Some(Some(player)) = players.remove(&label) {
                        player.stop();
                    }
                    let _ = reply.send(());
                    continue;
                }

                let mut new_player = Player::new_from_file_paths(tracks);
                new_player.set_effects(effects);

                let label_for_reporter = label.clone();
                let reporter = move |Report { time, .. }| {
                    if let Some(window) = app_handle.get_webview_window(&label_for_reporter) {
                        window.emit("UPDATE_PLAYHEAD", time).unwrap_or(());
                    }
                };

                new_player
                    .set_reporting(Arc::new(Mutex::new(reporter)), Duration::from_millis(100));
                new_player.pause();
                new_player.set_start_sink_chunks(1);
                new_player.set_start_buffer_ms(10.0);
                new_player.set_startup_fade_ms(5.0);
                new_player.set_max_sink_chunks(20);
                new_player.set_seek_fade_in_ms(50.0);
                new_player.set_seek_fade_out_ms(30.0);

                if let Some(Some(old_player)) = players.insert(label, Some(new_player)) {
                    old_player.stop();
                }
                let _ = reply.send(());
            }
            PlayerCommand::Clear { label, reply } => {
                if let Some(Some(player)) = players.remove(&label) {
                    player.stop();
                }
                let _ = reply.send(());
            }
            PlayerCommand::GetResumeState { label, reply } => {
                let out = players
                    .get(&label)
                    .and_then(|player| player.as_ref())
                    .map(|player| (player.is_playing(), player.get_time()))
                    .unwrap_or((false, 0.0));
                let _ = reply.send(out);
            }
            PlayerCommand::Play {
                label,
                effects,
                reply,
            } => {
                if let Some(Some(player)) = players.get_mut(&label) {
                    player.set_effects(effects);
                    player.play();
                }
                let _ = reply.send(());
            }
            PlayerCommand::Pause { label, reply } => {
                if let Some(Some(player)) = players.get(&label) {
                    player.pause();
                }
                let _ = reply.send(());
            }
            PlayerCommand::Stop { label, reply } => {
                if let Some(Some(player)) = players.get(&label) {
                    player.stop();
                }
                let _ = reply.send(());
            }
            PlayerCommand::Seek {
                label,
                position,
                reply,
            } => {
                if let Some(Some(player)) = players.get_mut(&label) {
                    player.seek(position);
                }
                let _ = reply.send(());
            }
            PlayerCommand::Shuffle { label, reply } => {
                let shuffled = if let Some(Some(player)) = players.get_mut(&label) {
                    player.shuffle();
                    true
                } else {
                    false
                };
                let _ = reply.send(shuffled);
            }
            PlayerCommand::GetPosition { label, reply } => {
                let value = players
                    .get(&label)
                    .and_then(|player| player.as_ref())
                    .map(|player| player.get_time())
                    .unwrap_or(0.0);
                let _ = reply.send(value);
            }
            PlayerCommand::GetDuration { label, reply } => {
                let value = players
                    .get(&label)
                    .and_then(|player| player.as_ref())
                    .map(|player| player.get_duration())
                    .unwrap_or(0.0);
                let _ = reply.send(value);
            }
            PlayerCommand::GetIds { label, reply } => {
                let value = players
                    .get(&label)
                    .and_then(|player| player.as_ref())
                    .map(|player| player.get_ids());
                let _ = reply.send(value);
            }
            PlayerCommand::SetVolume {
                label,
                volume,
                reply,
            } => {
                if let Some(Some(player)) = players.get_mut(&label) {
                    player.set_volume(volume);
                }
                let _ = reply.send(());
            }
            PlayerCommand::SetTrackMix {
                label,
                slot_index,
                level,
                pan,
                reply,
            } => {
                if let Some(Some(player)) = players.get_mut(&label) {
                    player.set_track_mix_inline(slot_index, level, pan);
                }
                let _ = reply.send(());
            }
            PlayerCommand::GetVolume { label, reply } => {
                let value = players
                    .get(&label)
                    .and_then(|player| player.as_ref())
                    .map(|player| player.get_volume())
                    .unwrap_or(1.0);
                let _ = reply.send(value);
            }
            PlayerCommand::SetEffectsInline {
                label,
                effects,
                reply,
            } => {
                let result = if let Some(Some(player)) = players.get_mut(&label) {
                    player.set_effects_inline(effects);
                    InlineEffectsResult::Applied
                } else {
                    InlineEffectsResult::NoPlayer
                };
                let _ = reply.send(result);
            }
            PlayerCommand::GetFlags { label, reply } => {
                let flags = if let Some(Some(player)) = players.get(&label) {
                    PlayerFlags {
                        exists: true,
                        is_playing: player.is_playing(),
                        is_paused: player.is_paused(),
                    }
                } else {
                    PlayerFlags {
                        exists: false,
                        is_playing: false,
                        is_paused: false,
                    }
                };
                let _ = reply.send(flags);
            }
            PlayerCommand::GetLevels { label, reply } => {
                let value = players
                    .get(&label)
                    .and_then(|player| player.as_ref())
                    .map(|player| player.get_levels())
                    .unwrap_or_else(|| vec![0.0, 0.0]);
                let _ = reply.send(value);
            }
            PlayerCommand::GetLevelsDb { label, reply } => {
                let value = players
                    .get(&label)
                    .and_then(|player| player.as_ref())
                    .map(|player| player.get_levels_db())
                    .unwrap_or_else(|| vec![f32::NEG_INFINITY, f32::NEG_INFINITY]);
                let _ = reply.send(value);
            }
            PlayerCommand::GetShuffleSchedule { label, reply } => {
                let value = players
                    .get(&label)
                    .and_then(|player| player.as_ref())
                    .map(|player| player.get_shuffle_schedule());
                let _ = reply.send(value);
            }
        }
    }
}

use std::sync::{Arc, Mutex};

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

pub fn player_play(window: &Window, state: &State<PlayerActorState>, effects: Vec<EffectSettings>) {
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
