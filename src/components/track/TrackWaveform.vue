<template>
  <div class="track">
    <div
      ref="overviewContainerRef"
      :id="`overview-container-${identifier}`"
      class="overview-container"
    >
      <canvas ref="canvasRef" class="waveform-canvas" @click="seek"></canvas>
      <div class="playhead"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useAudioStore } from '../../stores/audio'
import type { TrackFile } from '../../typings/tracks'

interface Props {
  track: TrackFile
  selected: boolean
}

interface SimplifiedPeaks {
  peaks: number[]
  zoom: number
  original_length: number
}

const audio = useAudioStore()
const props = defineProps<Props>()

const identifier = computed(() => `${props.track.parentId}-${props.track.id}`)

const canvasRef = ref<HTMLCanvasElement | null>(null)
const overviewContainerRef = ref<HTMLDivElement | null>(null)
const simplifiedPeaks = ref([] as SimplifiedPeaks[])

const peaksLength = computed(() => simplifiedPeaks.value[0]?.peaks.length || 0)

const zoomLevel = computed(() => {
  if (peaksLength.value === 0) return 1
  const originalLength = simplifiedPeaks.value[0]?.original_length || peaksLength.value
  const factor = originalLength / peaksLength.value
  return factor === 0 ? 1 : 100 / factor
})

const annotate = (index: number): boolean => {
  const zoom = zoomLevel.value

  let toAnnotate = false

  const setToAnnotate = (division: number) => {
    toAnnotate = Math.floor(index % (zoom * division)) === 0
  }

  if (zoom <= 3) setToAnnotate(30)
  else if (zoom <= 5) setToAnnotate(10)
  else if (zoom <= 15) setToAnnotate(5)
  else if (zoom <= 25) setToAnnotate(3)
  else if (zoom <= 100) setToAnnotate(2)
  else setToAnnotate(1)

  return toAnnotate
}

const getAnnotation = (index: number): string => {
  const seconds = index / zoomLevel.value
  return new Date(seconds * 1000).toISOString().slice(14, 19)
}

const playheadPosition = computed(() => {
  const factor = audio.clock * zoomLevel.value * 2
  return `${factor}px`
})

const getSimplifiedPeaks = async () => {
  return invoke<SimplifiedPeaks[]>('get_simplified_peaks', {
    fileId: props.track.id,
    zoom: audio.zoom.x,
  })
}

const drawWaveform = () => {
  const canvas = canvasRef.value
  const container = overviewContainerRef.value

  if (!canvas || !container || peaksLength.value === 0) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const width = Math.max(container.clientWidth, 1)
  const height = Math.max(container.clientHeight, 150)
  const dpr = window.devicePixelRatio || 1

  canvas.width = Math.floor(width * dpr)
  canvas.height = Math.floor(height * dpr)
  canvas.style.width = `${width}px`
  canvas.style.height = `${height}px`

  ctx.setTransform(dpr, 0, 0, dpr, 0, 0)
  ctx.clearRect(0, 0, width, height)

  ctx.fillStyle = 'white'
  ctx.fillRect(0, 0, width, height)

  const channels = simplifiedPeaks.value
  const channelCount = Math.max(channels.length, 1)
  const channelHeight = height / channelCount

  ctx.strokeStyle = 'rgba(116, 116, 116, 0.6)'
  ctx.lineWidth = 1

  channels.forEach((channel, channelIndex) => {
    const yTop = channelIndex * channelHeight
    const yMid = yTop + channelHeight / 2
    const maxAmplitude = channelHeight / 2 - 2

    ctx.beginPath()

    channel.peaks.forEach((peak, index) => {
      const x = index * 2 + 1
      if (x > width) return
      const amplitude = Math.min(Math.max(peak, 0), 1) * maxAmplitude
      ctx.moveTo(x, yMid - amplitude)
      ctx.lineTo(x, yMid + amplitude)
    })

    ctx.stroke()
  })

  ctx.strokeStyle = 'rgba(0, 0, 0, 0.1)'
  ctx.fillStyle = 'rgba(0, 0, 0, 0.4)'
  ctx.font = '11px Silkscreen, Segoe UI, Tahoma, Geneva, Verdana, sans-serif'
  ctx.textAlign = 'center'

  for (let i = 1; i <= peaksLength.value; i++) {
    if (!annotate(i)) continue

    const x = (i - 1) * 2
    if (x > width) break
    ctx.beginPath()
    ctx.moveTo(x, 0)
    ctx.lineTo(x, 10)
    ctx.moveTo(x, height)
    ctx.lineTo(x, height - 10)
    ctx.stroke()

    ctx.fillText(getAnnotation(i), x, height - 15)
  }
}

const updateSimplifiedPeaks = async () => {
  simplifiedPeaks.value = await getSimplifiedPeaks()
  await nextTick()
  drawWaveform()
}

const seek = (event: MouseEvent) => {
  const canvas = canvasRef.value
  if (!canvas) return

  const rect = canvas.getBoundingClientRect()
  const x = event.clientX - rect.left
  const seconds = x / zoomLevel.value / 2
  void audio.seek(seconds)
}

watch(
  () => audio.zoom.x,
  () => {
    void updateSimplifiedPeaks()
  },
)

watch(
  () => props.track.id,
  () => {
    void updateSimplifiedPeaks()
  },
)

onMounted(() => {
  void updateSimplifiedPeaks()
  window.addEventListener('resize', drawWaveform)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', drawWaveform)
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
