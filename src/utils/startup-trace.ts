import { invoke } from '@tauri-apps/api/core'

interface StartupTracePayload {
  stage: string
  elapsedMs: number
}

let hasWarnedTraceFailure = false

const traceToBackend = (stage: string, elapsedMs: number) => {
  const payload: StartupTracePayload = { stage, elapsedMs }

  // Tauri command signature is `startup_trace(payload: StartupTracePayload, ...)`,
  // so the invoke shape must include a `payload` key.
  void invoke('startup_trace', {
    payload,
  }).catch(() => {
    // Keep startup path resilient in non-Tauri/browser contexts while still surfacing
    // one warning for debugging when trace IPC is unavailable.
    if (!hasWarnedTraceFailure) {
      hasWarnedTraceFailure = true
      console.warn('[startup][web] failed to send trace payload to backend')
    }
  })
}

export const startupMark = (stage: string) => {
  if (typeof performance === 'undefined') return
  const elapsedMs = performance.now()
  performance.mark(`startup:${stage}`)
  console.info(`[startup][web] +${elapsedMs.toFixed(1)}ms ${stage}`)
  traceToBackend(stage, elapsedMs)
}
