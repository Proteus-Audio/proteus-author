<template>
  <div>KEY STROKES</div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount } from 'vue'
import { useAudioStore } from '../../stores/audio'
import { useTrackStore } from '../../stores/track'

const audio = useAudioStore()
const track = useTrackStore()

const keyListener = (e: KeyboardEvent) => {
  if ((e.target as HTMLElement).localName !== 'input') {
    const hasCommandModifier = e.metaKey || e.ctrlKey
    if (hasCommandModifier) {
      // Use KeyboardEvent.code for layout-independent shortcuts on macOS.
      if (e.code === 'Equal') {
        e.preventDefault()
        if (e.shiftKey) audio.zoomIn('y')
        else audio.zoomIn('x')
      }
      if (e.code === 'Minus') {
        e.preventDefault()
        if (e.shiftKey) audio.zoomOut('y')
        else audio.zoomOut('x')
      }
    }

    if (e.metaKey || e.ctrlKey || e.altKey) return

    if (e.key === ' ') {
      e.preventDefault()
      if (!audio.isPlaying && !track.initialised) {
        window.dispatchEvent(new Event('resize'))
        track.initialised = true
      }
      void audio.playPause()
    }
    if (e.key === 's') {
      e.preventDefault()
      void track.shuffle()
    }
    if (e.key === 'Enter') {
      e.preventDefault()
      void audio.seek(0)
    }
  }
}

onMounted(() => {
  window.addEventListener('keydown', keyListener)
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', keyListener)
})
</script>
