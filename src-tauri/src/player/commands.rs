use proteus_lib::container::play_settings::EffectSettings;
use proteus_lib::container::prot::Prot;
use tauri::{Emitter, Manager, State, Window};

use super::api::{
    player_duration, player_flags, player_ids, player_levels, player_levels_db, player_pause,
    player_play, player_position, player_resume_state, player_seek, player_set_effects_inline,
    player_set_track_mix, player_set_volume, player_shuffle, player_stop, player_volume,
    replace_window_player,
};
use super::mix::{
    build_paths_tracks, clamp_pan, clamp_track_level, format_shuffle_point_timestamp,
    parse_shuffle_point_seconds,
};
use super::types::{InlineEffectsResult, PlayerActorState, PlayerState};
use crate::project::{read_project, with_project_mut, WindowProjectState};
use crate::startup::{log_rust, StartupTraceState};

#[tauri::command]
pub async fn init_player(window: Window) {
    let start_of_process = std::time::Instant::now();
    let player_state: State<PlayerActorState> = window.state();
    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(&window, &project_state);

    let tracks_for_player = build_paths_tracks(&project);
    replace_window_player(
        &window,
        &player_state,
        tracks_for_player,
        project.effects.clone(),
    );
    window.emit("PLAYER_CHANGED", ()).unwrap_or(());

    log::debug!(
        "init_player took {}ms",
        start_of_process.elapsed().as_millis()
    );
}

#[tauri::command]
pub fn get_possible_combinations(window: Window) -> Option<String> {
    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(&window, &project_state);
    let tracks_for_player = build_paths_tracks(&project);

    if tracks_for_player.is_empty() {
        return Some(String::from("0"));
    }

    let prot = Prot::new_from_file_paths(tracks_for_player);
    prot.count_possible_combinations()
        .map(|count| count.to_string())
}

#[tauri::command]
pub async fn add_shuffle_point(track_id: u32, seconds: f64, window: Window) -> Vec<String> {
    let timestamp = format_shuffle_point_timestamp(seconds);

    {
        let project_state: State<WindowProjectState> = window.state();
        let mut updated = false;
        with_project_mut(&window, &project_state, |project| {
            let Some(track) = project.tracks.iter_mut().find(|track| track.id == track_id) else {
                return;
            };

            if !track.shuffle_points.contains(&timestamp) {
                track.shuffle_points.push(timestamp.clone());
                track.shuffle_points.sort_by(|a, b| {
                    parse_shuffle_point_seconds(a).total_cmp(&parse_shuffle_point_seconds(b))
                });
                track.shuffle_points.dedup();
            }
            updated = true;
        });
        if !updated {
            return Vec::new();
        }
    }

    let (resume_playback, current_time) = {
        let player_state: State<PlayerActorState> = window.state();
        player_resume_state(&window, &player_state)
    };

    init_player(window.clone()).await;

    if current_time > 0.0 {
        seek(current_time, window.clone()).await;
    }
    if resume_playback {
        play(window.clone()).await;
    }

    let project_state: State<WindowProjectState> = window.state();
    read_project(&window, &project_state)
        .tracks
        .iter()
        .find(|track| track.id == track_id)
        .map(|track| track.shuffle_points.clone())
        .unwrap_or_default()
}

#[tauri::command]
pub async fn remove_shuffle_point(
    track_id: u32,
    seconds: f64,
    tolerance_seconds: f64,
    window: Window,
) -> Vec<String> {
    let target = seconds.max(0.0);
    let tolerance = tolerance_seconds.max(0.0);

    {
        let project_state: State<WindowProjectState> = window.state();
        let mut updated = false;
        with_project_mut(&window, &project_state, |project| {
            let Some(track) = project.tracks.iter_mut().find(|track| track.id == track_id) else {
                return;
            };

            let mut best_index: Option<usize> = None;
            let mut best_distance = f64::INFINITY;

            for (index, point) in track.shuffle_points.iter().enumerate() {
                let distance = (parse_shuffle_point_seconds(point) - target).abs();
                if distance <= tolerance && distance < best_distance {
                    best_distance = distance;
                    best_index = Some(index);
                }
            }

            if let Some(index) = best_index {
                track.shuffle_points.remove(index);
            }
            updated = true;
        });
        if !updated {
            return Vec::new();
        }
    }

    let (resume_playback, current_time) = {
        let player_state: State<PlayerActorState> = window.state();
        player_resume_state(&window, &player_state)
    };

    init_player(window.clone()).await;

    if current_time > 0.0 {
        seek(current_time, window.clone()).await;
    }
    if resume_playback {
        play(window.clone()).await;
    }

    let project_state: State<WindowProjectState> = window.state();
    read_project(&window, &project_state)
        .tracks
        .iter()
        .find(|track| track.id == track_id)
        .map(|track| track.shuffle_points.clone())
        .unwrap_or_default()
}

#[tauri::command]
pub async fn play(window: Window) {
    let player_state: State<PlayerActorState> = window.state();
    let project_state: State<WindowProjectState> = window.state();
    let effects = read_project(&window, &project_state).effects;
    log::debug!("Setting effects: {:?}", effects);
    player_play(&window, &player_state, effects);
}

#[tauri::command]
pub async fn pause(window: Window) {
    let player_state: State<PlayerActorState> = window.state();
    player_pause(&window, &player_state);
}

#[tauri::command]
pub async fn stop(window: Window) {
    let player_state: State<PlayerActorState> = window.state();
    player_stop(&window, &player_state);
}

#[tauri::command]
pub async fn seek(position: f64, window: Window) {
    let player_state: State<PlayerActorState> = window.state();
    player_seek(&window, &player_state, position);
}

#[tauri::command]
pub async fn shuffle(window: Window) {
    log::debug!("shuffling");
    let player_state: State<PlayerActorState> = window.state();
    let shuffled = player_shuffle(&window, &player_state);
    if !shuffled {
        return;
    }

    log::debug!("Calling set_selections");
    set_selections(window);
}

#[tauri::command]
pub async fn get_position(window: Window) -> f64 {
    let player_state: State<PlayerActorState> = window.state();
    player_position(&window, &player_state)
}

#[tauri::command]
pub async fn get_duration(window: Window) -> f64 {
    let player_state: State<PlayerActorState> = window.state();
    player_duration(&window, &player_state)
}

#[tauri::command]
pub fn set_selections(window: Window) -> Vec<String> {
    log::debug!("setting selections");
    let player_state: State<PlayerActorState> = window.state();
    let urls = player_ids(&window, &player_state);
    let Some(urls) = urls else {
        return Vec::new();
    };

    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(&window, &project_state);

    let mut ids = Vec::new();

    for url in urls {
        let id = project
            .files
            .iter()
            .find(|f| f.path == url)
            .unwrap()
            .id
            .clone();

        ids.push(id);
    }

    log::debug!("ids: {:?}", ids);

    with_project_mut(&window, &project_state, |project| {
        for track in project.tracks.iter_mut() {
            track.selection = None;
        }
        for (track, id) in project.tracks.iter_mut().zip(ids.iter()) {
            if track.file_ids.contains(id) {
                track.selection = Some(id.clone());
            }
        }
    });

    ids
}

#[tauri::command]
pub fn set_volume(volume: f32, window: Window) {
    log::debug!("setting volume: {}", volume);
    let player_state: State<PlayerActorState> = window.state();
    player_set_volume(&window, &player_state, volume);
}

#[tauri::command]
pub async fn set_track_mix(track_id: u32, level: f32, pan: f32, window: Window) {
    let clamped_level = clamp_track_level(level);
    let clamped_pan = clamp_pan(pan);
    let project_state: State<WindowProjectState> = window.state();

    let mut slot_index: Option<usize> = None;
    with_project_mut(&window, &project_state, |project| {
        let mut playback_index = 0usize;
        for track in project.tracks.iter_mut() {
            let is_playback_track = !track.file_ids.is_empty();
            if track.id == track_id {
                track.level = clamped_level;
                track.pan = clamped_pan;
                if is_playback_track {
                    slot_index = Some(playback_index);
                }
                break;
            }
            if is_playback_track {
                playback_index += 1;
            }
        }
    });

    let Some(slot_index) = slot_index else {
        return;
    };

    let player_state: State<PlayerActorState> = window.state();
    player_set_track_mix(
        &window,
        &player_state,
        slot_index,
        clamped_level,
        clamped_pan,
    );
}

#[tauri::command]
pub fn get_volume(window: Window) -> f32 {
    let player_state: State<PlayerActorState> = window.state();
    player_volume(&window, &player_state)
}

#[tauri::command]
pub fn set_effects_chain(
    effects: Vec<EffectSettings>,
    window: Window,
    project_state: State<WindowProjectState>,
    startup_trace_state: State<StartupTraceState>,
) {
    with_project_mut(&window, &project_state, |project| {
        project.effects = effects.clone();
    });

    let player_state: State<PlayerActorState> = window.state();
    log::debug!("Setting effects: {:?}", effects);
    let result = player_set_effects_inline(&window, &player_state, effects.clone());
    if matches!(result, InlineEffectsResult::NoPlayer) && !effects.is_empty() {
        log_rust(
            &startup_trace_state,
            "player",
            "set_effects_chain called before player init (No player found)",
        );
    }
}

#[tauri::command]
pub fn get_play_state(window: Window) -> PlayerState {
    let player_state: State<PlayerActorState> = window.state();
    let flags = player_flags(&window, &player_state);
    if !flags.exists {
        return PlayerState::Stopped;
    }
    if flags.is_playing {
        return PlayerState::Playing;
    }
    if flags.is_paused {
        return PlayerState::Paused;
    }
    PlayerState::Stopped
}

#[tauri::command]
pub fn get_levels(window: Window) -> Vec<f32> {
    let player_state: State<PlayerActorState> = window.state();
    player_levels(&window, &player_state)
}

#[tauri::command]
pub fn get_levels_db(window: Window) -> Vec<f32> {
    let player_state: State<PlayerActorState> = window.state();
    player_levels_db(&window, &player_state)
}
