use serde::Deserialize;
use std::sync::Mutex;
use std::time::Instant;
use tauri::{State, Window};

pub struct StartupTraceState {
    app_started_at: Instant,
    last_log_elapsed_ms: Mutex<Option<f64>>,
}

pub fn create_startup_trace_state() -> StartupTraceState {
    StartupTraceState {
        app_started_at: Instant::now(),
        last_log_elapsed_ms: Mutex::new(None),
    }
}

pub fn elapsed_ms(state: &StartupTraceState) -> f64 {
    state.app_started_at.elapsed().as_secs_f64() * 1000.0
}

fn delta_since_last_ms(state: &StartupTraceState, now_ms: f64) -> Option<f64> {
    let mut guard = state.last_log_elapsed_ms.lock().unwrap();
    let previous = *guard;
    *guard = Some(now_ms);
    previous.map(|last| now_ms - last)
}

pub fn log_rust(state: &StartupTraceState, source: &str, stage: &str) {
    let now_ms = elapsed_ms(state);
    let delta = delta_since_last_ms(state, now_ms);
    let delta_part = delta
        .map(|value| format!(" (Δ{:.1}ms)", value))
        .unwrap_or_default();

    println!(
        "[startup][rust][{}] +{:.1}ms{} {}",
        source, now_ms, delta_part, stage
    );
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupTracePayload {
    pub stage: String,
    pub elapsed_ms: Option<f64>,
}

#[tauri::command]
pub fn startup_trace(
    payload: StartupTracePayload,
    window: Window,
    startup_trace_state: State<StartupTraceState>,
) {
    let rust_elapsed = elapsed_ms(&startup_trace_state);
    let delta = delta_since_last_ms(&startup_trace_state, rust_elapsed);
    let delta_part = delta
        .map(|value| format!(" (Δ{:.1}ms)", value))
        .unwrap_or_default();

    match payload.elapsed_ms {
        Some(web_elapsed) => {
            println!(
                "[startup][web][{}] +{:.1}ms{} (web +{:.1}ms) {}",
                window.label(),
                rust_elapsed,
                delta_part,
                web_elapsed,
                payload.stage
            );
        }
        None => {
            println!(
                "[startup][web][{}] +{:.1}ms{} {}",
                window.label(),
                rust_elapsed,
                delta_part,
                payload.stage
            );
        }
    }
}
