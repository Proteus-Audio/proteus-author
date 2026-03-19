use std::path::Path;

use proteus_lib::container::prot::PathsTrack;

use crate::project::ProjectSkeleton;

const TRACK_LEVEL_MAX: f32 = 10f32;

pub(super) fn build_paths_tracks(project: &ProjectSkeleton) -> Vec<PathsTrack> {
    let max_track_level = TRACK_LEVEL_MAX.powf(10.0 / 20.0);
    let clamp_level = |value: f32| value.clamp(0.0, max_track_level);
    let clamp_pan = |value: f32| value.clamp(-1.0, 1.0);

    project
        .tracks
        .iter()
        .filter_map(|track| {
            let mut file_paths: Vec<String> = track
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
                .collect();

            if file_paths.is_empty() {
                return None;
            }

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

            Some(PathsTrack {
                file_paths,
                level: clamp_level(track.level),
                pan: clamp_pan(track.pan),
                selections_count: 1,
                shuffle_points: track.shuffle_points.clone(),
            })
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
