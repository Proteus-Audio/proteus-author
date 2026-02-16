<template>
  <div class="grid grid-cols-[100px_100px_100px_110px_50px_50px_70px_70px] gap-4 text-right">
    <UButton
      v-if="!audio.isPlaying"
      icon="i-lucide-play"
      variant="ghost"
      color="neutral"
      @click="play"
    >
      play
    </UButton>
    <UButton v-else icon="i-lucide-pause" variant="ghost" color="neutral" @click="pause">
      pause
    </UButton>
    <UButton
      id="BaseTransportStop"
      icon="i-lucide-square"
      variant="ghost"
      color="neutral"
      @click="stop"
    >
      stop
    </UButton>
    <UButton
      id="BaseTransportShuffle"
      icon="i-lucide-shuffle"
      variant="ghost"
      color="neutral"
      @click="shuffle"
    >
      shuffle
    </UButton>
    <UButton
      id="BaseTransportFollowMode"
      :color="audio.followMode ? 'primary' : 'neutral'"
      :variant="audio.followMode ? 'outline' : 'ghost'"
      @click="toggleFollowMode"
    >
      follow {{ audio.followMode ? 'on' : 'off' }}
    </UButton>
    <UButton
      icon="i-lucide-zoom-in"
      variant="ghost"
      color="neutral"
      :disabled="zoomInDisabled"
      @click="zoomIn"
    />
    <UButton
      icon="i-lucide-zoom-out"
      variant="ghost"
      color="neutral"
      :disabled="zoomOutDisabled"
      @click="zoomOut"
    />
    <UButton icon="i-lucide-arrow-left" variant="ghost" color="neutral" @click="panLeft"
      >left</UButton
    >
    <UButton icon="i-lucide-arrow-right" variant="ghost" color="neutral" @click="panRight"
      >right</UButton
    >
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAudioStore } from '../../stores/audio'
import { useTrackStore } from '../../stores/track'

const audio = useAudioStore()
const track = useTrackStore()

const zoomInDisabled = computed(() => {
  return audio.getViewDuration <= 0.51
})

const zoomOutDisabled = computed(() => {
  if (audio.duration <= 0) return false
  return audio.getViewDuration >= audio.duration - 0.01
})

const play = () => {
  void audio.play()
}

const pause = () => {
  void audio.pause()
}

const stop = () => {
  void audio.stop()
}

const shuffle = () => {
  void track.shuffle()
}

const toggleFollowMode = () => audio.toggleFollowMode()
const zoomIn = () => audio.zoomIn()
const zoomOut = () => audio.zoomOut()
const panLeft = () => audio.panViewLeft(0.2)
const panRight = () => audio.panViewRight(0.2)
</script>
