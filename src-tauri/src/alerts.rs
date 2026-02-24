use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Runtime, Window};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertPayload {
    pub message: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
}

fn build_alert_payload(
    message: impl Into<String>,
    alert_type: impl Into<String>,
    id: Option<String>,
    loading: Option<bool>,
    replace: Option<bool>,
) -> AlertPayload {
    AlertPayload {
        message: message.into(),
        r#type: alert_type.into(),
        id,
        loading,
        replace,
    }
}

pub fn emit_alert_current_window(
    window: &Window,
    message: impl Into<String>,
    alert_type: impl Into<String>,
) {
    let payload = build_alert_payload(message, alert_type, None, None, None);
    let _ = window.emit("ALERT_CURRENT_WINDOW", payload);
}

pub fn emit_alert_current_window_keyed(
    window: &Window,
    id: impl Into<String>,
    message: impl Into<String>,
    alert_type: impl Into<String>,
    loading: bool,
) {
    let payload = build_alert_payload(
        message,
        alert_type,
        Some(id.into()),
        Some(loading),
        Some(true),
    );
    let _ = window.emit("ALERT_CURRENT_WINDOW", payload);
}

pub fn emit_alert_all_windows<R: Runtime>(
    app: &AppHandle<R>,
    message: impl Into<String>,
    alert_type: impl Into<String>,
) {
    let payload = build_alert_payload(message, alert_type, None, None, None);
    let _ = app.emit("ALERT_ALL_WINDOWS", payload);
}

#[tauri::command]
pub fn alert_current_window(window: Window, message: String, r#type: Option<String>) {
    emit_alert_current_window(
        &window,
        message,
        r#type.unwrap_or_else(|| "info".to_string()),
    );
}

#[tauri::command]
pub fn alert_all_windows(app: AppHandle, message: String, r#type: Option<String>) {
    emit_alert_all_windows(&app, message, r#type.unwrap_or_else(|| "info".to_string()));
}
