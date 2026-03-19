use std::path::Path;

use proteus_lib::container::prot::PathsTrack;

use crate::project::{ProjectSkeleton, TrackSkeleton};

const TRACK_LEVEL_MAX: f32 = 10f32;

#[derive(Clone, Copy)]
pub(super) struct InlineTrackMix {
    pub slot_index: usize,
    pub level: f32,
    pub pan: f32,
}

fn existing_track_file_paths(project: &ProjectSkeleton, track: &TrackSkeleton) -> Vec<String> {
    track
        .file_ids
        .iter()
        .filter_map(|id| {
            project
                .files
                .iter()
                .find(|f| f.id == *id)
                .map(|f| f.path.clone())
                .filter(|path| Path::new(path).exists())
        })
        .collect()
}

fn collect_playable_tracks(project: &ProjectSkeleton) -> Vec<(usize, Vec<String>)> {
    project
        .tracks
        .iter()
        .enumerate()
        .filter_map(|(index, track)| {
            let file_paths = existing_track_file_paths(project, track);
            if file_paths.is_empty() {
                None
            } else {
                Some((index, file_paths))
            }
        })
        .collect()
}

pub(crate) fn any_playable_track_soloed(project: &ProjectSkeleton) -> bool {
    collect_playable_tracks(project)
        .iter()
        .any(|(index, _)| project.tracks[*index].soloed)
}

pub(crate) fn effective_track_level(track: &TrackSkeleton, any_soloed: bool) -> f32 {
    if track.muted {
        return 0.0;
    }
    if any_soloed && !track.soloed {
        return 0.0;
    }
    clamp_track_level(track.level)
}

pub(super) fn build_paths_tracks(project: &ProjectSkeleton) -> Vec<PathsTrack> {
    let any_soloed = any_playable_track_soloed(project);

    collect_playable_tracks(project)
        .into_iter()
        .map(|(track_index, mut file_paths)| {
            let track = &project.tracks[track_index];

            // Keep the selected file first so the active take remains stable at init time.
            if let Some(selection_id) = &track.selection {
                if let Some(selected_path) = project
                    .files
                    .iter()
                    .find(|f| f.id == *selection_id)
                    .map(|f| f.path.clone())
                    .filter(|path| Path::new(path).exists())
                {
                    if let Some(index) = file_paths.iter().position(|path| path == &selected_path) {
                        if index > 0 {
                            file_paths.swap(0, index);
                        }
                    }
                }
            }

            PathsTrack {
                file_paths,
                level: effective_track_level(track, any_soloed),
                pan: clamp_pan(track.pan),
                selections_count: 1,
                shuffle_points: track.shuffle_points.clone(),
            }
        })
        .collect()
}

pub(super) fn build_inline_track_mixes(project: &ProjectSkeleton) -> Vec<InlineTrackMix> {
    let any_soloed = any_playable_track_soloed(project);

    collect_playable_tracks(project)
        .into_iter()
        .enumerate()
        .map(|(slot_index, (track_index, _))| {
            let track = &project.tracks[track_index];
            InlineTrackMix {
                slot_index,
                level: effective_track_level(track, any_soloed),
                pan: clamp_pan(track.pan),
            }
        })
        .collect()
}

pub(super) fn clamp_track_level(level: f32) -> f32 {
    let max_track_level = TRACK_LEVEL_MAX.powf(10.0 / 20.0);
    level.clamp(0.0, max_track_level)
}

pub(super) fn clamp_pan(pan: f32) -> f32 {
    pan.clamp(-1.0, 1.0)
}

pub(super) fn format_shuffle_point_timestamp(seconds: f64) -> String {
    let normalized = if seconds.is_finite() {
        seconds.max(0.0)
    } else {
        0.0
    };
    format!("{normalized:.3}")
}

pub(super) fn parse_shuffle_point_seconds(value: &str) -> f64 {
    let parts: Vec<&str> = value.trim().split(':').collect();
    if parts.is_empty() || parts.len() > 3 {
        return f64::INFINITY;
    }

    let seconds_component = parts
        .last()
        .and_then(|part| part.parse::<f64>().ok())
        .unwrap_or(f64::INFINITY);
    let minutes = if parts.len() >= 2 {
        parts[parts.len() - 2].parse::<f64>().unwrap_or(0.0)
    } else {
        0.0
    };
    let hours = if parts.len() == 3 {
        parts[0].parse::<f64>().unwrap_or(0.0)
    } else {
        0.0
    };

    (hours * 3600.0) + (minutes * 60.0) + seconds_component
}
