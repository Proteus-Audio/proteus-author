use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use proteus_lib::diagnostics::reporter::Report;
use proteus_lib::playback::player::Player;
use tauri::{Emitter, Manager};

use super::types::{InlineEffectsResult, PlayerActorState, PlayerCommand, PlayerFlags};
use crate::effects::decode_effects;

pub fn create_player_actor_state() -> PlayerActorState {
    let (tx, rx) = mpsc::channel::<PlayerCommand>();
    thread::spawn(move || player_worker(rx));
    PlayerActorState { tx }
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
                new_player.set_effects(decode_effects(&effects));

                let label_for_reporter = label.clone();
                let reporter = move |Report { time, .. }| {
                    if let Some(window) = app_handle.get_webview_window(&label_for_reporter) {
                        window.emit("UPDATE_PLAYHEAD", time).unwrap_or(());
                    }
                };

                new_player
                    .set_reporting(Arc::new(Mutex::new(reporter)), Duration::from_millis(100));
                new_player.pause();

                new_player.configure_for_live_authoring();
                new_player.set_max_sink_latency_ms(Some(100.0));
                new_player.set_max_sink_chunks(20);
                new_player.set_start_buffer_ms(10.0);

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
                    player.set_effects(decode_effects(&effects));
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
                    player.set_effects_inline(decode_effects(&effects));
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
