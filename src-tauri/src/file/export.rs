use crate::alerts::{emit_alert_current_window, emit_alert_current_window_keyed};
use crate::effects::decode_effects;
use crate::file::utils::{attachment_mime_for_path, split_arguments, unique_attachment_name};
use crate::project::{read_project, WindowProjectState};
use proteus_lib::container::play_settings::PlaySettingsContainer;
use proteus_lib::container::play_settings::{
    EffectSettings, PlaySettingsFile, PlaySettingsV2, PlaySettingsV2File, SettingsTrack,
};
use proteus_lib::dsp::effects::AudioEffect;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tauri::Manager;
use tauri::State;
use tauri::Window;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

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

    let handle = window.app_handle().clone();
    let alert_window = window.clone();

    save_dialog.save_file(move |file_path| {
        if file_path.is_none() {
            println!("No file selected");
            emit_alert_current_window(&alert_window, "Cancelled", "info");
            ()
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
        let mut reduced_file_list = Vec::new();

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
                    let attachment_name = unique_attachment_name(file_name, &mut used_attachment_names);
                    let attachment_ref = format!("attachment:{}", attachment_name);

                    convolution.settings.impulse_response = Some(attachment_ref.clone());
                    convolution.settings.impulse_response_attachment = Some(attachment_ref);
                    convolution.settings.impulse_response_path = None;

                    let mime = attachment_mime_for_path(ir_path).to_string();
                    ir_attachments.push((path, attachment_name, mime));
                }
            }
        }

        let encoded_effects: Vec<EffectSettings> = effects
            .into_iter()
            .filter_map(|effect| match serde_json::to_value(effect) {
                Ok(value) => Some(value),
                Err(err) => {
                    log::warn!("failed to serialize effect entry: {}", err);
                    None
                }
            })
            .collect();

        let mut play_settings = PlaySettingsV2 {
            effects: encoded_effects,
            tracks: Vec::new(),
        };

        for track in project.tracks.iter() {
            let mut settings_track = SettingsTrack {
                level: track.level,
                pan: track.pan,
                ids: Vec::new(),
                name: track.name.clone(),
                safe_name: track.name.clone(),
                selections_count: 1,
                shuffle_points: track.shuffle_points.clone(),
            };

            for file_id in &track.file_ids {
                let file = project.files.iter().find(|f| f.id == *file_id).unwrap().clone();

                if !reduced_file_list.contains(&file.path) {
                    reduced_file_list.push(file.path.clone());
                }

                let index = reduced_file_list
                    .iter()
                    .position(|r| r == &file.path)
                    .unwrap();
                settings_track.ids.push((index + 1) as u32);
            }

            play_settings.tracks.push(settings_track);
        }

        let mut input_list = String::new();
        let mut map_list = String::new();
        let mut metadata_list = String::new();

        for (index, file) in reduced_file_list.iter().enumerate() {
            input_list.push_str(&format!("-i \"{}\" ", file));
            map_list.push_str(&format!("-map {} ", index));
            metadata_list.push_str(&format!("-metadata:s:a:{} title=\"{}\" ", index, file));
        }

        let settings_file = PlaySettingsV2File {
            settings: PlaySettingsContainer::Nested {
                play_settings: play_settings,
            },
        };

        let settings_encoder = PlaySettingsFile::V2(settings_file);

        let json_settings = serde_json::to_string(&settings_encoder).unwrap();

        let output_dir = file_path.parent().unwrap().to_str().unwrap().to_string();
        let settings_file_path = format!("{}/play_settings.json", output_dir);
        let mut settings_file = File::create(settings_file_path.clone()).unwrap();
        settings_file.write_all(json_settings.as_bytes()).unwrap();

        print!("Settings Written: {:?}", json_settings);

        let output_file = file_path
            .to_str()
            .unwrap()
            .to_string()
            .replace(".prot", ".mka");

        let mut attachment_args = String::new();
        let mut attachment_index = 0;

        attachment_args.push_str(&format!(
            "-attach \"{}\" -metadata:s:t:{} mimetype=application/json -metadata:s:t:{} filename=play_settings.json ",
            settings_file_path, attachment_index, attachment_index
        ));
        attachment_index += 1;

        for (path, name, mime) in ir_attachments.iter() {
            attachment_args.push_str(&format!(
                "-attach \"{}\" -metadata:s:t:{} mimetype={} -metadata:s:t:{} filename=\"{}\" ",
                path, attachment_index, mime, attachment_index, name
            ));
            attachment_index += 1;
        }

        let out_command = format!(
            "-y {}{}{}{} {} \"{}\"",
            input_list,
            map_list,
            attachment_args,
            metadata_list,
            "-f matroska",
            output_file
        );

        println!("{}", out_command);
        println!("{:?}", split_arguments(out_command.as_str()));

        let shell = handle.shell();
        let (mut rx, ..) = shell
            .sidecar("proteus-author-ffmpeg")
            .expect("failed to create `proteus-author-ffmpeg` binary command")
            .args(split_arguments(out_command.as_str()))
            .spawn()
            .expect("Failed to spawn sidecar");

        let alert_window_for_task = alert_window.clone();
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        let text = String::from_utf8_lossy(&line);
                        println!("Line: {}", text.trim_end());
                    }
                    CommandEvent::Stderr(line) => {
                        let text = String::from_utf8_lossy(&line);
                        eprintln!("Error: {}", text.trim_end());
                    }
                    CommandEvent::Terminated(exit_status) => {
                        println!("Exit: {:#?}", exit_status);
                    }
                    _ => {}
                }
            }

            std::fs::remove_file(settings_file_path).unwrap();

            if std::path::Path::new(&output_file).exists() {
                std::fs::rename(output_file.clone(), output_file.replace(".mka", ".prot")).unwrap();
            }

            emit_alert_current_window_keyed(
                &alert_window_for_task,
                "export_prot",
                "Export Finished",
                "success",
                false,
            );
        });
    });
}
