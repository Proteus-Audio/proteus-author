<template>
  <div class="grid grid-cols-[100px_100px_100px_110px_50px_50px_70px_70px] gap-4 text-right">
    <UButton
      v-if="!audio.isPlaying"
      icon="dinkie-icons:right-black-triangle"
      variant="ghost"
      color="neutral"
      @click="play"
    >
      play
    </UButton>
    <UButton
      v-else
      icon="dinkie-icons:double-vertical-bar"
      variant="ghost"
      color="neutral"
      @click="pause"
    >
      pause
    </UButton>
    <UButton
      id="BaseTransportStop"
      icon="dinkie-icons:black-square-for-stop"
      variant="ghost"
      color="neutral"
      @click="stop"
    >
      stop
    </UButton>
    <UButton
      id="BaseTransportShuffle"
      icon="dinkie-icons:shuffle-arrows"
      variant="ghost"
      color="neutral"
      @click="shuffle"
    >
      shuffle
    </UButton>
    <UButton
      id="BaseTransportFollowMode"
      :color="audio.followMode ? 'primary' : 'neutral'"
      class="text-center"
      variant="outline"
      @click="toggleFollowMode"
    >
      <div class="w-full text-center">follow {{ audio.followMode ? 'on' : 'off' }}</div>
    </UButton>
    <UButton
      icon="dinkie-icons:right-magnifying-glass-filled"
      variant="ghost"
      color="neutral"
      :disabled="zoomOutDisabled"
      @click="zoomOut"
    />
    <UButton
      icon="dinkie-icons:right-magnifying-glass"
      variant="ghost"
      color="neutral"
      :disabled="zoomInDisabled"
      @click="zoomIn"
    />
    <UButton
      icon="dinkie-icons:black-left-double-triangle-with-vertical-bar"
      variant="ghost"
      color="neutral"
      @click="panLeft"
      >left</UButton
    >
    <UButton
      icon="dinkie-icons:black-right-double-triangle-with-vertical-bar"
      variant="ghost"
      color="neutral"
      @click="panRight"
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
