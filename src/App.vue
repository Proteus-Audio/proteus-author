<template>
  <div id="proteus-author">
    <Teleport to="head">
      <title>Proteus Author - {{ windowTitle }}</title>
    </Teleport>
    <UtilBase />
    <BaseContainer>
      <BaseAlertBox />
      <BaseTitle />
      <el-affix :offset="0">
        <BaseTransport />
      </el-affix>

      <div class="bin-container">
        <TrackBin v-for="track in trackStore.tracks" :track-id="track.id" :key="track.id" />
      </div>
      <div class="padding"></div>
    </BaseContainer>

    <EffectRack />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import EffectRack from './components/effects/EffectRack.vue'
import BaseContainer from './components/base/BaseContainer.vue'
import TrackBin from './components/track/TrackBin.vue'
import BaseTransport from './components/base/BaseTransport.vue'
import BaseAlertBox from './components/base/BaseAlertBox.vue'
import UtilBase from './components/util/UtilBase.vue'
import { useHeadStore } from './stores/head'
import { useTrackStore } from './stores/track'
import { useAudioStore } from './stores/audio'
import BaseTitle from './components/base/BaseTitle.vue'
import { ProjectSkeleton } from './typings/proteus'

const head = useHeadStore()
const trackStore = useTrackStore()
const audio = useAudioStore()

const windowTitle = computed(() => {
  return head.name.replace('.protproject', '')
})

onMounted(async () => {
  const urlSearchParams = new URLSearchParams(window.location.search)
  const params = Object.fromEntries(urlSearchParams.entries())

  // const data: ProjectSkeleton | undefined = await ipcRenderer.invoke('init', params.id)
  // if (data) head.load(data)

  trackStore.addEmptyTrackIfNone()
})

watch(audio.zoom, () => {
  window.dispatchEvent(new Event('resize'))
})
</script>

<style lang="scss">
body {
  margin: 0;
  font-family: 'Silkscreen', 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.shuffler {
  cursor: pointer;
  margin-bottom: 1em;
  display: block;

  &:hover {
    opacity: 0.7;
  }
}

.padding {
  display: inline-block;
  width: 1em;
  height: 1em;
}

.bin-container {
  width: 100%;
  overflow-x: scroll;
  border-radius: 0.5em;
}
</style>
