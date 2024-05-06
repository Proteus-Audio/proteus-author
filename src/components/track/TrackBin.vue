<template>
  <div
    v-bind="getRootProps()"
    class="track-bin"
    :class="{ drag: hovering, loading, clickable: fresh }"
    :style="`min-width: ${width}; ${padding}`"
    @click="
      () => {
        if (fresh) openFiles()
      }
    "
  >
    <div v-if="!fresh" class="bin">
      <BaseLoadingSpinner :message="loadingMessage" v-if="loading" class="loader" />
      <div class="bin-name">
        <InputAutoSizedText
          class="track-name"
          placeholder="Click to Add Name"
          v-model="trackName"
        />
        -
        <div type="text" class="selection-name">{{ selectedFile?.name }}</div>

        <el-button
          :icon="Folder"
          class="folder-button"
          @click="() => (folderOpen = !folderOpen)"
          text
        />
      </div>
      <div class="waveforms">
        <TrackWaveform
          v-if="selectedFile"
          :class="`waveform visible`"
          :key="selectedFile.id"
          :track="selectedFile"
          :selected="selectedFile.id === track.selection"
          >{{ selectedFile.name }}</TrackWaveform
        >
      </div>
      <el-drawer
        ref="folderContents"
        v-model="folderOpen"
        :title="`Track Bin Contents`"
        custom-class="drawer"
      >
        <div class="tracklist">
          <div v-for="id in track.file_ids" :key="id">
            {{ trackStore.getFileFromId(id)?.name }}

            <el-button :icon="Delete" class="closeButton" @click="() => removeFile(id)" text />
          </div>
        </div>
      </el-drawer>
    </div>

    <input v-bind="getInputProps()" />

    <span v-if="fresh" class="message clickable">
      <BaseLoadingSpinner v-if="loading" class="loader" />
      <p v-if="hovering">Drop the files here ...</p>
      <p v-else>
        Drag 'n' drop some files here, or click to select files
        <span class="error">{{ error }}</span>
      </p>
    </span>
    <span v-else>
      <p class="error" v-if="!!error">{{ error }}</p>
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'

import { useDropzone } from 'vue3-dropzone'
import { useTrackStore } from '../../stores/track'
import TrackWaveform from './TrackWaveform.vue'

import { Folder, Delete } from '@element-plus/icons-vue'
import InputAutoSizedText from '../input/InputAutoSizedText.vue'
import { useAudioStore } from '../../stores/audio'
import { DropFileSkeleton } from '../../typings/tracks'
import { open } from '@tauri-apps/api/dialog'
import { UnlistenFn } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'
import BaseLoadingSpinner from '../base/BaseLoadingSpinner.vue'
import { invoke } from '@tauri-apps/api'
import { useAlertStore } from '../../stores/alerts'
// import Button from "element-plus";

interface Props {
  trackId: number
}

const unlisten: UnlistenFn[] = []

const props = defineProps<Props>()

const trackStore = useTrackStore()
const audio = useAudioStore()
const alerts = useAlertStore()

const track = computed(() => trackStore.getOrCreateTrackFromId(props.trackId))

const width = computed((): string => {
  return audio.duration === 0 ? '100%' : `${audio.zoom.x * audio.duration + 30}px`
})

const padding = computed((): string => {
  return track.value.file_ids.length > 0 ? '' : 'margin: 0;'
})

const folderOpen = ref(false)
const error = ref('')
const windowHover = ref(false)
const loading = ref(false)
const loadingMessage = ref('')

const hovering = computed(() => {
  return isDragActive.value && windowHover.value
})

const trackName = computed({
  get: () => {
    const index = trackStore.getTrackIndexFromId(props.trackId)
    return trackStore.tracks[index].name || ''
  },
  set: (name: string) => {
    return trackStore.setTrackName(props.trackId, name)
  },
})

const selectedFile = computed(() => {
  if (!track.value.selection) return undefined
  return trackStore.getFileFromId(track.value.selection)
})

const loadFiles = async (files: string[]) => {
  loading.value = true
  const acceptableFiles = files.filter((file) => /(?:.mp3|.wav)$/.test(file))
  if (acceptableFiles.length !== files.length) {
    alerts.addAlert('Only WAV and MP3 files are accepted at the moment.', 'warning')
  }

  if (acceptableFiles.length > 0) {
    // const fileData: DropFileSkeleton[] = []
    for (let i = 0; i < acceptableFiles.length; i++) {
      const filePath = acceptableFiles[i]
      console.log(filePath, /(?:.mp3|.wav)$/.test(filePath))

      const file = (await invoke('register_file', {
        filePath,
        trackId: props.trackId,
      })) as DropFileSkeleton

      console.log(file)

      loadingMessage.value = `Processing ${file.name}`

      await trackStore.addFileToTrackBinary(file, props.trackId)
    }

    await invoke('init_player')
    console.log('finished processing')

    trackStore.shuffle()

    trackStore.sync()
  }
  loading.value = false
}

const removeFile = (id: string) => trackStore.removeFileFromTrack(id, props.trackId)

const fresh = computed(() => {
  const isFresh = track.value.file_ids.length === 0
  return isFresh
})

const { getRootProps, getInputProps, isDragActive } = useDropzone({
  accept: ['audio/mpeg', 'audio/wav'],
  noClick: true,
})

const openFiles = async () => {
  const files = await open({
    multiple: true,
    filters: [{ name: 'Audio Files', extensions: ['wav', 'mp3'] }],
  })
  if (!files) return
  loadFiles(typeof files === 'string' ? [files] : files)
}

onMounted(async () => {
  unlisten.push(
    await appWindow.onFileDropEvent((event) => {
      if (event.payload.type === 'hover') {
        windowHover.value = true
      } else if (event.payload.type === 'drop' && isDragActive.value) {
        console.log('file drop', event)
        windowHover.value = false
        loadFiles(event.payload.paths as string[])
      } else {
        windowHover.value = false
      }
    }),
  )
})

onUnmounted(() => {
  console.log('unmounting')
  unlisten.forEach((unlistener) => unlistener())
})
</script>

<style lang="scss" scoped>
.clickable {
  cursor: pointer;
}

.track-bin {
  background: rgba(0, 0, 0, 0.1);
  padding: 1em;
  margin-bottom: 0.5em;
  position: relative;
  // border-radius: 0.5em;
  &.drag {
    background: rgba(0, 0, 0, 0.2);
  }

  &.loading {
    :deep(.channels .channel .annotation .timestamp) {
      background: rgba(0, 0, 0, 0.2);
    }
  }
  /* .loader {
  }*/
  .error {
    color: rgb(189, 50, 50);
  }

  .bin {
    .bin-name {
      .track-name,
      .selection-name {
        display: inline-block;
      }
    }

    .flex {
      display: flex;
      // grid-template-columns: 1fr 44px;
    }

    .waveforms {
      display: inline-block;
      position: relative;
      width: 100%;
      height: 150px;

      .waveform {
        position: absolute;
        width: 100%;
        display: block;
        top: 0;
        &.hidden {
          pointer-events: none;
          opacity: 0;
        }
      }
    }

    .folder-button {
      margin: 0 0.5em;
      margin-top: auto;
    }

    &:deep(.drawer) {
      min-width: 300px;

      .tracklist {
        display: grid;
        //   grid-template-columns: 1fr 45px;
      }

      //   .closeButton {
      //     display: inline-block;
      //   }
    }
  }
}
</style>
