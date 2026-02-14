<template>
  <div class="track">
    <div
      ref="overviewContainerRef"
      :id="`overview-container-${identifier}`"
      class="overview-container"
    >
      <canvas
        ref="canvasRef"
        class="waveform-canvas"
        :class="{ 'add-shuffle-point-mode': audio.addShufflePointMode }"
        @click="seek"
        @wheel.prevent="onWheel"
      ></canvas>
      <div class="playhead"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useAudioStore } from '../../stores/audio'
import { useTrackStore } from '../../stores/track'
import type { TrackFile } from '../../typings/tracks'

interface Props {
  track: TrackFile
  selected: boolean
}

const audio = useAudioStore()
const trackStore = useTrackStore()
const props = defineProps<Props>()

const identifier = computed(() => `${props.track.parentId}-${props.track.id}`)

const canvasRef = ref<HTMLCanvasElement | null>(null)
const overviewContainerRef = ref<HTMLDivElement | null>(null)
const waveformChannels = ref<number[][]>([])
const canvasWidthPx = ref(1)
const MIN_FETCH_INTERVAL_MS = 16
const MAX_FETCH_INTERVAL_MS = 72
const ADAPTIVE_INTERVAL_SMOOTHING = 0.25

const viewDuration = computed(() => Math.max(audio.getViewDuration, 0.001))
const verticalScale = computed(() => Math.max(audio.getYScale, 0.1))

const playheadPosition = computed(() => {
  const width = canvasWidthPx.value
  const ratio = (audio.clock - audio.getViewStart) / viewDuration.value
  const x = Math.min(Math.max(ratio * width, 0), width)
  return `${x}px`
})

const trackShufflePoints = computed(() => {
  const track = trackStore.getTrackFromId(props.track.parentId)
  return track?.shuffle_points || []
})

const formatTimestamp = (seconds: number): string => {
  const total = Math.max(0, Math.floor(seconds))
  const mins = Math.floor(total / 60)
  const secs = total % 60
  return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`
}

const getTickStep = (secondsPerFrame: number): number => {
  const targetTicks = 8
  const raw = secondsPerFrame / targetTicks
  const options = [0.5, 1, 2, 5, 10, 15, 30, 60, 120, 300, 600]
  return options.find((v) => v >= raw) || 600
}

const parseShufflePointSeconds = (value: string): number | null => {
  const trimmed = value.trim()
  if (!trimmed) return null

  const parts = trimmed.split(':')
  if (parts.length > 3) return null

  if (parts.length === 1) {
    const seconds = Number(parts[0])
    return Number.isFinite(seconds) && seconds >= 0 ? seconds : null
  }

  const seconds = Number(parts[parts.length - 1])
  const minutes = Number(parts[parts.length - 2])
  const hours = parts.length === 3 ? Number(parts[0]) : 0

  if (![seconds, minutes, hours].every((value) => Number.isFinite(value) && value >= 0)) {
    return null
  }

  return hours * 3600 + minutes * 60 + seconds
}

const drawWaveform = () => {
  const canvas = canvasRef.value
  const container = overviewContainerRef.value
  if (!canvas || !container) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const width = Math.max(container.clientWidth, 1)
  const height = Math.max(container.clientHeight, 150)
  const dpr = window.devicePixelRatio || 1

  canvasWidthPx.value = width
  canvas.width = Math.floor(width * dpr)
  canvas.height = Math.floor(height * dpr)
  canvas.style.width = `${width}px`
  canvas.style.height = `${height}px`

  ctx.setTransform(dpr, 0, 0, dpr, 0, 0)
  ctx.clearRect(0, 0, width, height)

  ctx.fillStyle = 'white'
  ctx.fillRect(0, 0, width, height)

  const channels = waveformChannels.value
  const channelCount = Math.max(channels.length, 1)
  const channelHeight = height / channelCount
  const yScale = Number(verticalScale.value)
  const minPeak = 0.008

  ctx.strokeStyle = 'rgba(116, 116, 116, 0.6)'
  ctx.lineWidth = 1

  channels.forEach((channel, channelIndex) => {
    if (channel.length === 0) return
    const yTop = channelIndex * channelHeight
    const yMid = yTop + channelHeight / 2
    const maxAmplitude = channelHeight / 2 - 2
    const stepX = width / channel.length

    ctx.beginPath()
    channel.forEach((peak, index) => {
      const x = index * stepX + stepX / 2
      peak = yScale * peak
      const normalizedPeak = peak < minPeak ? minPeak : peak > 1 ? 1 : peak
      const amplitude = normalizedPeak * maxAmplitude
      ctx.moveTo(x, yMid - amplitude)
      ctx.lineTo(x, yMid + amplitude)
    })
    ctx.stroke()
  })

  const start = audio.getViewStart
  const end = audio.getViewEnd
  const span = Math.max(end - start, 0.001)
  const tickStep = getTickStep(span)
  const firstTick = Math.ceil(start / tickStep) * tickStep

  ctx.strokeStyle = 'rgba(0, 0, 0, 0.1)'
  ctx.fillStyle = 'rgba(0, 0, 0, 0.4)'
  ctx.font = '11px Silkscreen, Segoe UI, Tahoma, Geneva, Verdana, sans-serif'
  ctx.textAlign = 'center'

  for (let tick = firstTick; tick <= end; tick += tickStep) {
    const ratio = (tick - start) / span
    const x = ratio * width
    ctx.beginPath()
    ctx.moveTo(x, 0)
    ctx.lineTo(x, 10)
    ctx.moveTo(x, height)
    ctx.lineTo(x, height - 10)
    ctx.stroke()
    ctx.fillText(formatTimestamp(tick), x, height - 15)
  }

  // Draw shuffle point indicators on top of waveform and time ticks.
  const shufflePointTimes = trackShufflePoints.value
    .map(parseShufflePointSeconds)
    .filter((time): time is number => time !== null)
    .filter((time) => time >= start && time <= end)

  ctx.strokeStyle = 'rgba(196, 50, 50, 0.9)'
  ctx.fillStyle = 'rgba(196, 50, 50, 0.95)'
  ctx.lineWidth = 1

  for (const time of shufflePointTimes) {
    const x = ((time - start) / span) * width
    ctx.beginPath()
    ctx.moveTo(x, 0)
    ctx.lineTo(x, height)
    ctx.stroke()

    ctx.beginPath()
    ctx.moveTo(x - 4, 0)
    ctx.lineTo(x + 4, 0)
    ctx.lineTo(x, 7)
    ctx.closePath()
    ctx.fill()
  }
}

let updateTimer: number | null = null
let updateQueued = false
let updateInFlight = false
let lastUpdateAt = 0
let activeRequestId = 0
let adaptiveFetchIntervalMs = 33

const clamp = (value: number, min: number, max: number) => {
  return Math.min(Math.max(value, min), max)
}

const runWaveformUpdate = async () => {
  if (updateInFlight) {
    updateQueued = true
    return
  }

  const updateStartedAt = performance.now()
  updateInFlight = true
  const requestId = ++activeRequestId
  lastUpdateAt = updateStartedAt

  try {
    const width = Math.max(overviewContainerRef.value?.clientWidth || 0, 1)
    const targetPeaks = Math.max(Math.floor(width / 2), 64)

    const channels = await invoke<number[][]>('get_waveform_peaks', {
      fileId: props.track.id,
      startSeconds: audio.getViewStart,
      endSeconds: audio.getViewEnd,
      targetPeaks,
    })

    // Drop stale responses if a newer request has already started.
    if (requestId !== activeRequestId) return

    waveformChannels.value = channels
    await nextTick()
    drawWaveform()
  } finally {
    const elapsed = performance.now() - updateStartedAt
    const targetInterval = clamp(elapsed * 0.9, MIN_FETCH_INTERVAL_MS, MAX_FETCH_INTERVAL_MS)
    adaptiveFetchIntervalMs =
      adaptiveFetchIntervalMs * (1 - ADAPTIVE_INTERVAL_SMOOTHING) +
      targetInterval * ADAPTIVE_INTERVAL_SMOOTHING

    updateInFlight = false
    if (updateQueued) {
      updateQueued = false
      queueWaveformUpdate(false)
    }
  }
}

const queueWaveformUpdate = (immediate = false) => {
  if (updateTimer !== null) {
    if (!immediate) return
    window.clearTimeout(updateTimer)
    updateTimer = null
  }

  if (immediate) {
    void runWaveformUpdate()
    return
  }

  const elapsed = performance.now() - lastUpdateAt
  const delay = elapsed >= adaptiveFetchIntervalMs ? 0 : adaptiveFetchIntervalMs - elapsed
  updateTimer = window.setTimeout(() => {
    updateTimer = null
    void runWaveformUpdate()
  }, delay)
}

const seek = (event: MouseEvent) => {
  const canvas = canvasRef.value
  if (!canvas) return

  const rect = canvas.getBoundingClientRect()
  const x = event.clientX - rect.left
  const ratio = x / Math.max(rect.width, 1)
  const seconds = audio.getViewStart + ratio * viewDuration.value
  if (audio.addShufflePointMode) {
    void trackStore.addShufflePoint(props.track.parentId, seconds)
    return
  }
  void audio.seek(seconds)
}

const onWheel = (event: WheelEvent) => {
  const canvas = canvasRef.value
  if (!canvas) return

  let delta = event.deltaX
  if (Math.abs(delta) < 0.01 && event.shiftKey) {
    delta = event.deltaY
  }
  if (Math.abs(delta) < 0.01) return

  const width = Math.max(canvas.clientWidth, 1)
  const fraction = delta / width
  audio.panViewByFraction(fraction)
}

watch(
  () => [audio.getViewStart, audio.getViewEnd, props.track.id],
  () => {
    queueWaveformUpdate(false)
  },
)

watch(
  () => audio.getYScale,
  () => {
    drawWaveform()
  },
)

watch(
  () => trackShufflePoints.value,
  () => {
    drawWaveform()
  },
  { deep: true },
)

onMounted(() => {
  queueWaveformUpdate(true)
  window.addEventListener('resize', onResize)
})

const onResize = () => {
  queueWaveformUpdate(true)
}

onBeforeUnmount(() => {
  if (updateTimer !== null) {
    window.clearTimeout(updateTimer)
    updateTimer = null
  }
  window.removeEventListener('resize', onResize)
})
</script>

<style lang="scss" scoped>
.track {
  background-color: rgba(0, 0, 0, 0.1);

  .overview-container {
    position: relative;
    min-height: 150px;
    width: 100%;
    overflow: hidden;
    background: white;
  }

  .waveform-canvas {
    display: block;
    cursor: pointer;
  }

  .waveform-canvas.add-shuffle-point-mode {
    cursor: copy;
  }

  .playhead {
    position: absolute;
    pointer-events: none;
    top: 0;
    left: v-bind(playheadPosition);
    height: 100%;
    width: 1px;
    background-color: #7474746e;
  }
}
</style>
