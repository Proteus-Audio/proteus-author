<template>
  <div id="proteus-author">
    <Teleport to="head">
      <title>Proteus Author - {{ windowTitle }}</title>
    </Teleport>
    <UtilBase />
    <div class="app-layout">
      <div class="app-main">
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

      <aside class="app-meter">
        <BaseLevelMeter vertical />
      </aside>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import BaseAlertBox from './components/base/BaseAlertBox.vue'
import BaseContainer from './components/base/BaseContainer.vue'
import BaseLevelMeter from './components/base/BaseLevelMeter.vue'
import BaseTitle from './components/base/BaseTitle.vue'
import BaseTransport from './components/base/BaseTransport.vue'
import EffectRack from './components/effects/EffectRack.vue'
import TrackBin from './components/track/TrackBin.vue'
import UtilBase from './components/util/UtilBase.vue'
import { useAlertStore } from './stores/alerts'
import { useAudioStore } from './stores/audio'
import { useHeadStore } from './stores/head'
import { useMenuStore } from './stores/menu'
import { useTrackStore } from './stores/track'
import type { AlertType, ProjectSkeleton } from './typings/proteus'

const head = useHeadStore()
const trackStore = useTrackStore()
const audio = useAudioStore()
const alerts = useAlertStore()
const menu = useMenuStore()

const windowTitle = computed(() => {
  return head.name.replace('.protproject', '')
})

const unlisteners = ref<UnlistenFn[]>([])

watch(
  [trackStore.tracks, audio.effects],
  async () => {
    console.log(await head.logChanges())
  },
  { deep: true },
)

watch(
  audio.effects,
  async () => {
    await audio.syncEffects()
  },
  { deep: true, immediate: true },
)

const handleSaveFile = async () => {
  console.log('saving file')
  const response = await invoke<ProjectSkeleton | null>('save_file', {
    newProject: head.projectState(),
  })

  if (response) {
    head.name = response.name || head.name
    head.path = response.location
    alerts.addAlert('Saved file', 'success')
  } else {
    alerts.addAlert('Failed to save file', 'error')
  }
}

const handleSaveFileAs = async () => {
  const response = await invoke<ProjectSkeleton | null>('save_file_as', {
    newProject: head.projectState(),
  })

  if (response) {
    head.name = response.name || head.name
    head.path = response.location
    alerts.addAlert('Saved file', 'success')
  } else {
    alerts.addAlert('Failed to save file', 'error')
  }
}

const handleStartExport = async () => {
  console.log('exporting')
  await invoke('export_prot', { project: head.projectState() })
}

onMounted(async () => {
  await menu.init()

  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  const fileLoaded = await listen('FILE_LOADED', (event) => {
    console.log('file loaded', event)
    const project = event?.payload as ProjectSkeleton
    if (project.location) alerts.addAlert('Loading project…', 'info')
    void head.load()
  })
  unlisteners.value.push(fileLoaded)

  const saveFile = await listen('SAVE_FILE', () => {
    void handleSaveFile()
  })
  unlisteners.value.push(saveFile)

  const saveFileAs = await listen('SAVE_FILE_AS', () => {
    void handleSaveFileAs()
  })
  unlisteners.value.push(saveFileAs)

  const startExport = await listen('START_EXPORT', () => {
    void handleStartExport()
  })
  unlisteners.value.push(startExport)

  const alert = await listen('ALERT', (event) => {
    const { message, type } = event.payload as {
      message: string
      type: AlertType
    }
    alerts.addAlert(message, type)
  })
  unlisteners.value.push(alert)

  const exporting = await listen('EXPORTING', (event) => {
    const message = event.payload as string
    alerts.addAlert(message, 'info')
  })
  unlisteners.value.push(exporting)

  const updatePlayhead = await listen('UPDATE_PLAYHEAD', (event) => {
    const time = event.payload as number
    audio.setClock(time)
  })
  unlisteners.value.push(updatePlayhead)

  trackStore.addEmptyTrackIfNone()

  console.log(await invoke('get_play_state'))

  await trackStore.sync()
})

onUnmounted(() => {
  unlisteners.value.forEach((unlistener) => {
    unlistener()
  })
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

#proteus-author {
  --meter-width: 72px;
}

.app-layout {
  min-height: 100vh;
}

.app-main {
  padding-right: var(--meter-width);
}

.app-meter {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  width: var(--meter-width);
  background: #f6f6f6;
  border-left: 1px solid #d8d8d8;
}

#effect-rack {
  width: calc(100% - var(--meter-width));
  right: var(--meter-width);
}
</style>
