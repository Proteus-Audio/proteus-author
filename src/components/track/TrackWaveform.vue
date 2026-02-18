<template>
  <div class="bg-black/10">
    <div
      ref="overviewContainerRef"
      :id="`overview-container-${identifier}`"
      class="relative min-h-[150px] w-full overflow-hidden bg-white"
    >
      <canvas
        ref="canvasRef"
        class="block cursor-pointer"
        :class="{
          'cursor-copy': audio.shufflePointToolMode,
          'cursor-not-allowed': audio.shufflePointToolMode && hoveringShufflePoint,
        }"
        @click="seek"
        @mousemove="onMouseMove"
        @mouseleave="onMouseLeave"
        @wheel.prevent="onWheel"
      ></canvas>
      <div
        class="pointer-events-none absolute top-0 h-full w-px bg-[#7474746e]"
        :style="{ left: playheadPosition }"
      ></div>
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

interface WaveformSegment {
  start_seconds: number
  end_seconds: number
  file_name: string
  file_end_seconds: number
  left_boundary_is_shuffle_point: boolean
  right_boundary_is_shuffle_point: boolean
}

interface TrackWaveformView {
  channels: number[][]
  segments: WaveformSegment[]
}

const audio = useAudioStore()
const trackStore = useTrackStore()
const props = defineProps<Props>()

const identifier = computed(() => `${props.track.parentId}-${props.track.id}`)

const canvasRef = ref<HTMLCanvasElement | null>(null)
const overviewContainerRef = ref<HTMLDivElement | null>(null)
const waveformChannels = ref<number[][]>([])
const waveformSegments = ref<WaveformSegment[]>([])
const canvasWidthPx = ref(1)
const hoveringShufflePoint = ref(false)
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

const shufflePointSeconds = computed(() =>
  trackShufflePoints.value
    .map(parseShufflePointSeconds)
    .filter((time): time is number => time !== null && Number.isFinite(time) && time >= 0),
)

const findNearestShufflePointSeconds = (
  seconds: number,
  toleranceSeconds: number,
): number | null => {
  let nearest: number | null = null
  let nearestDistance = Number.POSITIVE_INFINITY

  for (const point of shufflePointSeconds.value) {
    const distance = Math.abs(point - seconds)
    if (distance <= toleranceSeconds && distance < nearestDistance) {
      nearest = point
      nearestDistance = distance
    }
  }

  return nearest
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

  // Shade timeline regions where the currently displayed file has ended.
  const start = audio.getViewStart
  const end = audio.getViewEnd
  const span = Math.max(end - start, 0.001)
  for (const segment of waveformSegments.value) {
    const sectionStart = Math.max(segment.start_seconds, start)
    const sectionEnd = Math.min(segment.end_seconds, end)
    if (sectionEnd <= sectionStart) continue

    const pastFileStart = Math.max(sectionStart, segment.file_end_seconds)
    if (pastFileStart >= sectionEnd) continue

    const xStart = ((pastFileStart - start) / span) * width
    const xEnd = ((sectionEnd - start) / span) * width
    const shadeWidth = xEnd - xStart
    if (shadeWidth <= 0) continue

    ctx.fillStyle = 'rgba(120, 120, 120, 0.16)'
    ctx.fillRect(xStart, 0, shadeWidth, height)
  }

  const channels = waveformChannels.value
  const channelCount = Math.max(channels.length, 1)
  const channelHeight = height / channelCount
  const yScale = Number(verticalScale.value)
  const minPeak = 0.008

  const validRanges = waveformSegments.value
    .map((segment) => {
      const rangeStart = Math.max(segment.start_seconds, start)
      const rangeEnd = Math.min(segment.end_seconds, segment.file_end_seconds, end)
      return rangeEnd > rangeStart ? ([rangeStart, rangeEnd] as const) : null
    })
    .filter((range): range is readonly [number, number] => range !== null)

  const isInValidRange = (time: number): boolean => {
    for (const [rangeStart, rangeEnd] of validRanges) {
      if (time >= rangeStart && time <= rangeEnd) return true
    }
    return false
  }

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
      const time = start + ((index + 0.5) / channel.length) * span
      if (!isInValidRange(time)) return

      const x = index * stepX + stepX / 2
      const scaledPeak = peak * yScale
      const normalizedPeak = scaledPeak < minPeak ? minPeak : scaledPeak > 1 ? 1 : scaledPeak
      if (normalizedPeak <= 0) return
      const amplitude = normalizedPeak * maxAmplitude
      ctx.moveTo(x, yMid - amplitude)
      ctx.lineTo(x, yMid + amplitude)
    })
    ctx.stroke()
  })

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
  const shufflePointTimes = shufflePointSeconds.value.filter((time) => time >= start && time <= end)

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

  ctx.fillStyle = 'rgba(0, 0, 0, 0.78)'
  ctx.textAlign = 'center'
  ctx.textBaseline = 'middle'
  ctx.font = '10px Silkscreen, Segoe UI, Tahoma, Geneva, Verdana, sans-serif'

  for (const segment of waveformSegments.value) {
    const sectionStart = Math.max(segment.start_seconds, start)
    const sectionEnd = Math.min(segment.end_seconds, end)
    if (sectionEnd <= sectionStart) continue

    const xStart = ((sectionStart - start) / span) * width
    const xEnd = ((sectionEnd - start) / span) * width
    const sectionWidth = xEnd - xStart
    if (sectionWidth < 28) continue

    const textY = 14
    const text = segment.file_name
    const horizontalPadding = 6
    const leftIsPoint = segment.left_boundary_is_shuffle_point
    const rightIsPoint = segment.right_boundary_is_shuffle_point

    let textAlign: CanvasTextAlign
    let textX: number
    if (leftIsPoint && rightIsPoint) {
      textAlign = 'center'
      textX = xStart + sectionWidth / 2
    } else if (rightIsPoint) {
      textAlign = 'right'
      textX = xEnd - horizontalPadding
    } else if (leftIsPoint) {
      textAlign = 'left'
      textX = xStart + horizontalPadding
    } else {
      // No visible shuffle points in-region: center in the canvas.
      textAlign = 'center'
      textX = width / 2
    }

    ctx.fillStyle = 'rgba(0, 0, 0, 0.78)'
    ctx.textAlign = textAlign
    ctx.fillText(text, textX, textY)
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

    const view = await invoke<TrackWaveformView>('get_track_waveform_peaks', {
      trackId: props.track.parentId,
      startSeconds: audio.getViewStart,
      endSeconds: audio.getViewEnd,
      targetPeaks,
    })

    // Drop stale responses if a newer request has already started.
    if (requestId !== activeRequestId) return

    waveformChannels.value = view.channels
    waveformSegments.value = view.segments || []
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
  if (audio.shufflePointToolMode) {
    const pixelTolerance = 8
    const toleranceSeconds = (pixelTolerance / Math.max(rect.width, 1)) * viewDuration.value
    const nearest = findNearestShufflePointSeconds(seconds, toleranceSeconds)
    if (nearest !== null) {
      void trackStore.removeShufflePoint(props.track.parentId, nearest, toleranceSeconds)
      return
    }
    void trackStore.addShufflePoint(props.track.parentId, seconds)
    return
  }
  void audio.seek(seconds)
}

const onMouseMove = (event: MouseEvent) => {
  if (!audio.shufflePointToolMode) {
    hoveringShufflePoint.value = false
    return
  }

  const canvas = canvasRef.value
  if (!canvas) return

  const rect = canvas.getBoundingClientRect()
  const x = event.clientX - rect.left
  const ratio = x / Math.max(rect.width, 1)
  const seconds = audio.getViewStart + ratio * viewDuration.value
  const pixelTolerance = 8
  const toleranceSeconds = (pixelTolerance / Math.max(rect.width, 1)) * viewDuration.value
  hoveringShufflePoint.value = findNearestShufflePointSeconds(seconds, toleranceSeconds) !== null
}

const onMouseLeave = () => {
  hoveringShufflePoint.value = false
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

watch(
  () => audio.shufflePointToolMode,
  (enabled) => {
    if (!enabled) hoveringShufflePoint.value = false
  },
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
