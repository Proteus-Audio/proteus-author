import { invoke } from '@tauri-apps/api/core'

const traceToBackend = (stage: string, elapsedMs: number) => {
  void invoke('startup_trace', {
    stage,
    elapsedMs,
  }).catch(() => {
    // Ignore tracing failures in non-Tauri contexts.
  })
}

export const startupMark = (stage: string) => {
  if (typeof performance === 'undefined') return
  const elapsedMs = performance.now()
  console.info(`[startup][web] +${elapsedMs.toFixed(1)}ms ${stage}`)
  traceToBackend(stage, elapsedMs)
}
