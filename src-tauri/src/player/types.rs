use std::sync::mpsc::Sender;

use proteus_lib::container::play_settings::EffectSettings;
use proteus_lib::container::prot::PathsTrack;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

pub type ShuffleSchedule = Vec<(f64, Vec<Vec<String>>)>;

pub struct PlayerActorState {
    pub(super) tx: Sender<PlayerCommand>,
}

impl PlayerActorState {
    pub(super) fn send(&self, command: PlayerCommand) {
        let _ = self.tx.send(command);
    }
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

#[derive(Serialize, Deserialize)]
pub enum PlayerState {
    Playing,
    Paused,
    Stopped,
}

pub(super) enum PlayerCommand {
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
