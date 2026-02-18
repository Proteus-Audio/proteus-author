<template>
  <div
    ref="bin"
    class="relative mb-2 w-full overflow-hidden bg-black/10 p-4"
    :class="[
      hovering ? 'bg-black/20' : '',
      loading ? 'pointer-events-none' : '',
      fresh ? 'cursor-pointer' : '',
    ]"
    :style="padding"
    @click="
      () => {
        if (fresh) openFiles()
      }
    "
  >
    <div v-if="!fresh" class="relative">
      <BaseLoadingSpinner v-if="loading" :message="loadingMessage" class="loader" :inset="-4" />

      <div class="flex items-center gap-2">
        <InputAutoSizedText
          v-model="trackName"
          class="inline-block"
          placeholder="Click to Add Name"
        />

        <UButton
          icon="i-lucide-folder"
          variant="ghost"
          color="neutral"
          class="mb-4"
          @click.stop="toggleFolderOpen"
        />
      </div>

      <div class="grid h-[150px] w-full grid-cols-[minmax(0,1fr)_84px] gap-2">
        <div class="relative min-w-0 h-full">
          <TrackWaveform
            v-if="selectedFile"
            :key="selectedFile.id"
            class="absolute top-0 block w-full"
            :track="selectedFile"
            :selected="selectedFile.id === track.selection"
            >{{ selectedFile.name }}</TrackWaveform
          >
        </div>
        <DigitalTrackMix v-model:level="trackLevel" v-model:pan="trackPan" />
      </div>

      <UDrawer
        v-model:open="folderOpen"
        title="Track Bin Contents"
        direction="right"
        :handle="false"
      >
        <template #content>
          <div class="grid min-w-[300px] gap-2 p-4">
            <div
              v-for="id in track.file_ids"
              :key="id"
              class="grid grid-cols-[1fr_auto] items-center gap-2"
            >
              <span>{{ trackStore.getFileFromId(id)?.name }}</span>
              <UButton
                icon="i-lucide-trash-2"
                variant="ghost"
                color="error"
                @click="() => removeFile(id)"
              />
            </div>
          </div>
        </template>
      </UDrawer>
    </div>

    <span v-if="fresh">
      <BaseLoadingSpinner v-if="loading" class="loader" :inset="-4" />
      <p v-if="hovering">Drop the files here ...</p>
      <p v-else>
        Drag 'n' drop some files here, or click to select files
        <span class="text-red-700">{{ error }}</span>
      </p>
    </span>
    <span v-else>
      <p v-if="!!error" class="text-red-700">{{ error }}</p>
    </span>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import type { Event, UnlistenFn } from '@tauri-apps/api/event'
import { type DragDropEvent, Window } from '@tauri-apps/api/window'
import { computed, defineAsyncComponent, onMounted, onUnmounted, ref } from 'vue'
import { useAlertStore } from '../../stores/alerts'
import { useTrackStore } from '../../stores/track'
import type { DropFileSkeleton } from '../../typings/tracks'
import BaseLoadingSpinner from '../base/BaseLoadingSpinner.vue'
import { DigitalTrackMix } from '../digital'
import InputAutoSizedText from '../input/InputAutoSizedText.vue'

const TrackWaveform = defineAsyncComponent(() => import('./TrackWaveform.vue'))

interface Props {
  trackId: number
}

const unlisten: UnlistenFn[] = []

const props = defineProps<Props>()

const trackStore = useTrackStore()
const alerts = useAlertStore()
const bin = ref<HTMLElement | null>(null)

const track = computed(() => trackStore.getOrCreateTrackFromId(props.trackId))

const padding = computed((): string => {
  return track.value.file_ids.length > 0 ? '' : 'margin: 0;'
})

const folderOpen = ref(false)
const error = ref('')
const binHover = ref(false)
const loading = ref(false)
const loadingMessage = ref('')
const binBounds = ref({ left: 0, top: 0, right: 0, bottom: 0 })

const hovering = computed(() => {
  return binHover.value
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

const trackLevel = computed({
  get: () => track.value.level ?? 1,
  set: (value: number) => {
    trackStore.setTrackLevel(props.trackId, value)
  },
})

const trackPan = computed({
  get: () => track.value.pan ?? 0,
  set: (value: number) => {
    trackStore.setTrackPan(props.trackId, value)
  },
})

const selectedFile = computed(() => {
  if (!track.value.selection) return undefined
  const file = trackStore.getFileFromId(track.value.selection)
  if (!file) return undefined
  return { ...file, parentId: props.trackId }
})

const loadFiles = async (files: string[]) => {
  loading.value = true
  const acceptableFiles = files.filter((file) => /(?:.mp3|.wav)$/.test(file))
  if (acceptableFiles.length !== files.length) {
    console.log(acceptableFiles, files)
    alerts.addAlert('Only WAV and MP3 files are accepted at the moment.', 'warning')
  }

  if (acceptableFiles.length > 0) {
    for (let i = 0; i < acceptableFiles.length; i++) {
      const filePath = acceptableFiles[i]
      console.log(filePath, /(?:.mp3|.wav)$/.test(filePath))

      const file = await invoke<DropFileSkeleton>('register_file', {
        filePath,
        trackId: props.trackId,
      })

      console.log(file)

      loadingMessage.value = `Processing ${file.name}`

      trackStore.addFileToTrackBinary(file, props.trackId)
    }

    await invoke('init_player')
    console.log('finished processing')

    await trackStore.shuffle()
    await trackStore.sync()
  }
  loading.value = false
}

const removeFile = (id: string) => trackStore.removeFileFromTrack(id, props.trackId)

const fresh = computed(() => {
  const isFresh = track.value.file_ids.length === 0
  return isFresh
})

const toggleFolderOpen = () => {
  folderOpen.value = !folderOpen.value
}

const openFiles = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const files = await open({
    multiple: true,
    filters: [{ name: 'Audio Files', extensions: ['wav', 'mp3'] }],
  })
  if (!files) return
  console.log(files)
  await loadFiles(files)
}

const calcBinBounds = () => {
  const scroll = document.documentElement.scrollTop
  if (!bin.value) return { left: 0, top: 0 - scroll, right: 0, bottom: 0 - scroll }

  return {
    left: bin.value.offsetLeft,
    top: bin.value.offsetTop - scroll,
    right: bin.value.offsetLeft + bin.value.offsetWidth,
    bottom: bin.value.offsetTop + bin.value.offsetHeight - scroll,
  }
}

const checkBinHover = (position: { x: number; y: number }) => {
  return (
    position.x > binBounds.value.left &&
    position.x < binBounds.value.right &&
    position.y > binBounds.value.top &&
    position.y < binBounds.value.bottom
  )
}

onMounted(async () => {
  const appWindow = Window.getCurrent()
  binBounds.value = calcBinBounds()

  unlisten.push(
    await appWindow.onDragDropEvent((event: Event<DragDropEvent>) => {
      binBounds.value = calcBinBounds()
      if (event.payload.type === 'over' && checkBinHover(event.payload.position)) {
        binHover.value = true
      } else if (event.payload.type === 'drop' && binHover.value) {
        console.log('file drop', event)
        binHover.value = false
        void loadFiles(event.payload.paths)
      } else {
        binHover.value = false
      }
    }),
  )
})

onUnmounted(() => {
  console.log('unmounting')
  unlisten.forEach((unlistener) => {
    unlistener()
  })
})
</script>
