<template>
  <UApp>
    <div
      id="proteus-author"
      class="min-h-screen"
      :style="{
        '--meter-width': '154px',
        '--effect-rack-height': effectRackHeight,
      }"
    >
      <Teleport to="head">
        <title>Proteus Author - {{ windowTitle }}</title>
      </Teleport>

      <div class="min-h-screen">
        <div class="pr-[var(--meter-width)]">
          <BaseContainer>
            <BaseAlertBox />
            <BaseTitle />

            <div class="sticky top-0 z-30 bg-zinc-50 py-2 backdrop-blur-sm">
              <BaseTransport />
            </div>

            <div class="w-full overflow-hidden rounded-lg">
              <TrackBin v-for="track in trackStore.tracks" :key="track.id" :track-id="track.id" />
            </div>

            <div class="mt-3 text-[0.95rem] opacity-85">
              Unique playback combinations: {{ formattedPossibleCombinations }}
            </div>

            <div class="inline-block size-4"></div>
          </BaseContainer>

          <div ref="effectRackRef">
            <EffectRack />
          </div>
        </div>

        <aside
          class="fixed top-0 right-0 w-[var(--meter-width)] border-l border-zinc-300 bg-zinc-100 transition-[bottom] duration-300"
          :style="{ bottom: 'var(--effect-rack-height)' }"
        >
          <div class="grid h-full grid-cols-[1fr_54px]">
            <BaseLevelMeter vertical />
            <DigitalFader />
          </div>
        </aside>
      </div>
    </div>
  </UApp>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { Window } from '@tauri-apps/api/window'
import { useElementHover } from '@vueuse/core'
import { computed, defineAsyncComponent, onMounted, onUnmounted, ref, watch } from 'vue'
import BaseAlertBox from './components/base/BaseAlertBox.vue'
import BaseContainer from './components/base/BaseContainer.vue'
import BaseLevelMeter from './components/base/BaseLevelMeter.vue'
import BaseTitle from './components/base/BaseTitle.vue'
import BaseTransport from './components/base/BaseTransport.vue'
import { DigitalFader } from './components/digital'
import EffectRack from './components/effects/EffectRack.vue'
import { useAppShortcuts } from './composables/useAppShortcuts'
import { useAlertStore } from './stores/alerts'
import { useAudioStore } from './stores/audio'
import { useHeadStore } from './stores/head'
import { useTrackStore } from './stores/track'
import type { AlertType, ProjectSkeleton } from './typings/proteus'
import { startupMark } from './utils/startup-trace'

const TrackBin = defineAsyncComponent(() => import('./components/track/TrackBin.vue'))

const head = useHeadStore()
const trackStore = useTrackStore()
const audio = useAudioStore()
const alerts = useAlertStore()
const { registerShortcuts, unregisterShortcuts } = useAppShortcuts()
startupMark('App.vue:setup-start')

const windowTitle = computed(() => {
  return head.name.replace('.protproject', '')
})

const formattedPossibleCombinations = computed(() => {
  const raw = trackStore.possibleCombinations
  if (!/^\d+$/.test(raw)) return raw

  try {
    return BigInt(raw).toLocaleString()
  } catch {
    return raw
  }
})

const unlisteners = ref<UnlistenFn[]>([])
const startupHydrating = ref(true)

watch(
  [() => trackStore.tracks, () => audio.effects],
  async () => {
    if (startupHydrating.value) return
    console.log(await head.logChanges())
  },
  { deep: true },
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

const effectRackRef = ref<HTMLElement | null>(null)
const effectRackHover = useElementHover(effectRackRef)
const effectRackHeight = computed(() => (effectRackHover.value ? `7rem` : `5rem`))

const registerWindowListeners = async (
  appWindow: Window,
  runIfFocused: (action: () => void | Promise<void>) => Promise<void>,
) => {
  const listeners = await Promise.all([
    appWindow.listen('FILE_LOADED', (event) => {
      console.log('file loaded', event)
      const project = event?.payload as ProjectSkeleton
      if (project.location) alerts.addAlert('Loading project…', 'info')
      void (async () => {
        startupHydrating.value = true
        try {
          await head.load()
        } finally {
          startupHydrating.value = false
        }
      })()
    }),
    appWindow.listen('SAVE_FILE', () => {
      void runIfFocused(handleSaveFile)
    }),
    appWindow.listen('SAVE_FILE_AS', () => {
      void runIfFocused(handleSaveFileAs)
    }),
    appWindow.listen('OPEN_FILE', () => {
      void runIfFocused(async () => {
        await invoke('open_file')
      })
    }),
    appWindow.listen('START_EXPORT', () => {
      void runIfFocused(handleStartExport)
    }),
    appWindow.listen('ALERT_CURRENT_WINDOW', (event) => {
      const { message, type, id, loading, replace } = event.payload as {
        message: string
        type: AlertType
        id?: string
        loading?: boolean
        replace?: boolean
      }
      if (replace && id) {
        alerts.upsertAlert(id, message, type, { loading })
      } else {
        alerts.addAlert(message, type, { id, loading })
      }
    }),
    appWindow.listen('ALERT_ALL_WINDOWS', (event) => {
      const { message, type, id, loading, replace } = event.payload as {
        message: string
        type: AlertType
        id?: string
        loading?: boolean
        replace?: boolean
      }
      if (replace && id) {
        alerts.upsertAlert(id, message, type, { loading })
      } else {
        alerts.addAlert(message, type, { id, loading })
      }
    }),
    appWindow.listen('UPDATE_PLAYHEAD', (event) => {
      const time = event.payload as number
      audio.setClock(time)
    }),
    appWindow.listen('MENU_ZOOM_IN', () => {
      void runIfFocused(() => {
        audio.zoomIn('x')
      })
    }),
    appWindow.listen('MENU_ZOOM_OUT', () => {
      void runIfFocused(() => {
        audio.zoomOut('x')
      })
    }),
    appWindow.listen('MENU_ZOOM_IN_VERTICAL', () => {
      void runIfFocused(() => {
        audio.zoomIn('y')
      })
    }),
    appWindow.listen('MENU_ZOOM_OUT_VERTICAL', () => {
      void runIfFocused(() => {
        audio.zoomOut('y')
      })
    }),
    appWindow.listen('MENU_PAN_LEFT', () => {
      void runIfFocused(() => {
        audio.panViewLeft(0.2)
      })
    }),
    appWindow.listen('MENU_PAN_RIGHT', () => {
      void runIfFocused(() => {
        audio.panViewRight(0.2)
      })
    }),
    appWindow.listen('MENU_FOLLOW_MODE', (event) => {
      void runIfFocused(() => {
        const payload = event.payload as { enabled?: boolean }
        audio.setFollowMode(!!payload.enabled)
      })
    }),
    appWindow.listen('MENU_SHUFFLE_POINT_TOOL_MODE', (event) => {
      void runIfFocused(() => {
        const payload = event.payload as { enabled?: boolean }
        audio.setShufflePointToolMode(!!payload.enabled)
      })
    }),
  ])

  unlisteners.value.push(...listeners)
}

const runDeferredStartup = async () => {
  startupMark('App.vue:deferred-startup-begin')
  registerShortcuts()
  const appWindow = Window.getCurrent()
  const runIfFocused = async (action: () => void | Promise<void>) => {
    const focused = await appWindow.isFocused()
    if (!focused) return
    await action()
  }

  await registerWindowListeners(appWindow, runIfFocused)
  startupMark('App.vue:listeners-registered')

  trackStore.addEmptyTrackIfNone()

  startupMark('App.vue:before-get-play-state')
  console.log(await invoke('get_play_state'))
  startupMark('App.vue:after-get-play-state')

  startupMark('App.vue:before-track-sync')
  await trackStore.sync()
  startupMark('App.vue:after-track-sync')

  startupHydrating.value = false
  startupMark('App.vue:startup-hydration-complete')

  requestAnimationFrame(() => {
    startupMark('App.vue:first-frame-after-mounted-work')
  })
}

onMounted(() => {
  startupMark('App.vue:onMounted-start')
  requestAnimationFrame(() => {
    void runDeferredStartup()
  })
})

onUnmounted(() => {
  unregisterShortcuts()
  unlisteners.value.forEach((unlistener) => {
    unlistener()
  })
})

watch(
  () => [audio.getViewStart, audio.getViewEnd],
  () => {
    window.dispatchEvent(new Event('resize'))
  },
)
watch(
  () => audio.zoom,
  () => {
    window.dispatchEvent(new Event('resize'))
  },
  { deep: true },
)

watch(
  () => audio.followMode,
  (enabled) => {
    void invoke('set_follow_mode_menu', { enabled })
  },
)

watch(
  () => audio.shufflePointToolMode,
  (enabled) => {
    void invoke('set_shuffle_point_tool_mode_menu', { enabled })
  },
)

watch(
  () => audio.clock,
  (time) => {
    if (!audio.followMode) return
    if (!audio.isPlaying) return

    const viewStart = audio.getViewStart
    const viewEnd = audio.getViewEnd
    const viewDuration = audio.getViewDuration

    if (viewDuration <= 0) return
    if (time < viewStart || time > viewEnd) return

    const ratio = (time - viewStart) / viewDuration
    if (ratio < 0.8) return

    const targetPlayheadRatio = 0.35
    const nextStart = time - viewDuration * targetPlayheadRatio
    const nextEnd = nextStart + viewDuration
    audio.setViewRange(nextStart, nextEnd)
  },
)
</script>

<style>
button {
  cursor: pointer;
}
</style>
