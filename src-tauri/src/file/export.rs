use crate::alerts::{emit_alert_current_window, emit_alert_current_window_keyed};
use crate::effects::decode_effects;
use crate::file::utils::{attachment_mime_for_path, unique_attachment_name};
use crate::player::mix::{any_playable_track_soloed, effective_track_level};
use crate::project::{read_project, WindowProjectState};
use proteus_lib::container::play_settings::{EffectSettings, PlaySettingsPayload, SettingsTrack};
use proteus_lib::dsp::effects::AudioEffect;
use proteus_lib::mux::{ProtMuxAttachment, ProtMuxInput, ProtMuxTrackInput};
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::State;
use tauri::Window;
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Serialize)]
struct ExportPlaySettingsFile {
    encoder_version: String,
    play_settings: PlaySettingsPayload,
}

impl From<PlaySettingsPayload> for ExportPlaySettingsFile {
    fn from(play_settings: PlaySettingsPayload) -> Self {
        Self {
            encoder_version: "3".to_string(),
            play_settings,
        }
    }
}

#[tauri::command]
pub fn export_prot(window: Window, project_state: State<WindowProjectState>) {
    let project = read_project(&window, &project_state);
    let file_name = project.name.clone().unwrap_or("export".to_string()) + ".prot";
    let save_dialog = window
        .dialog()
        .file()
        .add_filter("Proteus Audio", &["prot"])
        .set_title("Save Project")
        .set_file_name(file_name.as_str());

    let alert_window = window.clone();

    save_dialog.save_file(move |file_path| {
        if file_path.is_none() {
            println!("No file selected");
            emit_alert_current_window(&alert_window, "Cancelled", "info");
            return;
        }

        let file_path = match file_path.clone().unwrap().into_path() {
            Ok(path) => path,
            Err(err) => {
                println!("Invalid file path: {:?}", err);
                emit_alert_current_window(&alert_window, "Cancelled", "info");
                return;
            }
        };
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        emit_alert_current_window_keyed(
            &alert_window,
            "export_prot",
            format!("Exporting {}", file_name),
            "info",
            true,
        );
        let mut mux_tracks: Vec<ProtMuxTrackInput> = Vec::new();

        let mut effects = decode_effects(&project.effects);
        let mut ir_attachments: Vec<(String, String, String)> = Vec::new();
        let mut used_attachment_names = HashSet::new();

        for effect in effects.iter_mut() {
            if let AudioEffect::ConvolutionReverb(convolution) = effect {
                let mut path = convolution.settings.impulse_response_path.clone();
                if path.is_none() {
                    if let Some(raw) = convolution.settings.impulse_response.clone() {
                        if !raw.starts_with("attachment:") {
                            let candidate = raw.strip_prefix("file:").unwrap_or(&raw).to_string();
                            if Path::new(&candidate).exists() {
                                path = Some(candidate);
                            }
                        }
                    }
                }

                if let Some(path) = path {
                    let ir_path = Path::new(&path);
                    if !ir_path.exists() {
                        println!("Impulse response not found: {}", path);
                        continue;
                    }

                    let file_name = ir_path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("impulse_response.wav");
                    let attachment_name =
                        unique_attachment_name(file_name, &mut used_attachment_names);
                    let attachment_ref = format!("attachment:{}", attachment_name);

                    convolution.settings.impulse_response = Some(attachment_ref.clone());
                    convolution.settings.impulse_response_attachment = Some(attachment_ref);
                    convolution.settings.impulse_response_path = None;

                    let mime = attachment_mime_for_path(ir_path).to_string();
                    ir_attachments.push((path, attachment_name, mime));
                }
            }
        }

        let encoded_effects: Vec<EffectSettings> =
            effects.into_iter().map(EffectSettings::from).collect();

        let mut play_settings = PlaySettingsPayload {
            effects: encoded_effects,
            tracks: Vec::new(),
        };
        let any_soloed = any_playable_track_soloed(&project);

        for track in project.tracks.iter() {
            let mut settings_track = SettingsTrack {
                level: effective_track_level(track, any_soloed),
                pan: track.pan,
                ids: Vec::new(),
                name: track.name.clone(),
                safe_name: track.name.clone(),
                selections_count: 1,
                shuffle_points: track.shuffle_points.clone(),
            };

            for file_id in &track.file_ids {
                let file = project
                    .files
                    .iter()
                    .find(|f| f.id == *file_id)
                    .unwrap()
                    .clone();

                if !mux_tracks
                    .iter()
                    .any(|track| track.source_path == PathBuf::from(&file.path))
                {
                    mux_tracks.push(ProtMuxTrackInput {
                        source_path: PathBuf::from(&file.path),
                        title: Some(file.path.clone()),
                    });
                }

                let index = mux_tracks
                    .iter()
                    .position(|mux_track| mux_track.source_path == PathBuf::from(&file.path))
                    .unwrap();
                settings_track.ids.push((index + 1) as u32);
            }

            play_settings.tracks.push(settings_track);
        }

        let settings_encoder = ExportPlaySettingsFile::from(play_settings);

        let play_settings_json = serde_json::to_vec(&settings_encoder).unwrap();
        let output_file = file_path.clone();
        let project_name = project.name.clone();
        let alert_window_for_task = alert_window.clone();
        tauri::async_runtime::spawn(async move {
            let mux_output_file = output_file.clone();
            let mux_result = tauri::async_runtime::spawn_blocking(move || {
                (|| {
                    let attachments = ir_attachments
                        .into_iter()
                        .map(|(path, name, mime)| {
                            fs::read(&path)
                                .map(|data| ProtMuxAttachment::new(name, mime, data))
                                .map_err(|err| {
                                    format!("Failed to read attachment {}: {}", path, err)
                                })
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    proteus_lib::mux::create_prot(ProtMuxInput {
                        output: mux_output_file,
                        tracks: mux_tracks,
                        play_settings_json,
                        attachments,
                        title: project_name,
                        writing_app: Some("proteus-author".to_string()),
                    })
                    .map_err(|err| err.to_string())
                })()
            })
            .await
            .unwrap_or_else(|err| Err(format!("Export task failed: {}", err)));

            match mux_result {
                Ok(()) => {
                    emit_alert_current_window_keyed(
                        &alert_window_for_task,
                        "export_prot",
                        "Export Finished",
                        "success",
                        false,
                    );
                }
                Err(err) => {
                    eprintln!("Export failed: {}", err);
                    if output_file.exists() {
                        if let Err(remove_err) = fs::remove_file(&output_file) {
                            eprintln!(
                                "Failed to remove incomplete export {}: {}",
                                output_file.display(),
                                remove_err
                            );
                        }
                    }
                    emit_alert_current_window_keyed(
                        &alert_window_for_task,
                        "export_prot",
                        format!("Export failed: {}", err),
                        "error",
                        false,
                    );
                }
            }
        });
    });
}
