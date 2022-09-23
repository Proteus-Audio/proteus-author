<template>
  <div>KEY STROKES</div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount } from 'vue'
import { toneMaster } from '../../assets/toneMaster'
import { useAudioStore } from '../../stores/audio'
import { useTrackStore } from '../../stores/track'

const audio = useAudioStore()
const track = useTrackStore()

const keyListener = (e: KeyboardEvent) => {
  if ((e.target as HTMLElement).localName !== 'input') {
    if (e.metaKey) {
      if (e.key === '=') {
        e.preventDefault()
        audio.zoomIn()
      }
      if (e.key === '-') {
        e.preventDefault()
        audio.zoomOut()
      }
    }
    if (e.metaKey || e.ctrlKey || e.altKey) return

    if (e.key === ' ') {
      e.preventDefault()
      if (!audio.isPlaying && !track.initialised) {
        window.dispatchEvent(new Event('resize'))
        track.initialised = true
      }
      audio.playPause()
    }
    if (e.key === 's') {
      e.preventDefault()
      track.shuffle()
    }
    if (e.key === 'Enter') {
      e.preventDefault()
      toneMaster.seek(0)
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
