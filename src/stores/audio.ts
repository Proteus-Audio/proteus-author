import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import * as Tone from 'tone'
import { computed, type Ref, ref, watch as vueWatch } from 'vue'
import {
  createEffect,
  type EffectChainItem,
  effectChainFromPayload,
  getEffectLabel,
  serializeEffectChainForBackend,
} from '../assets/effects'
import { toneMaster } from '../assets/toneMaster'
import type { AudioEffectPayload, AudioEffectType } from '../typings/effects'
import { useAlertStore } from './alerts'
import { useTrackStore } from './track'

export const useAudioStore = defineStore('prot', () => {
  const alert = useAlertStore()
  const track = useTrackStore()

  /////////////
  //  STORE  //
  /////////////

  const playing = ref(false)
  const currentTime = ref(0)
  const scale = ref(20 as number)
  const duration = ref(0)
  const zoom = ref({ y: 1 })
  const view = ref({ start: 0, end: 10 })
  const effects = ref([] as EffectChainItem[])
  const clock: Ref<number> = ref(0.0)
  const levelsDb = ref([-60, -60] as number[])

  const nextEffectId = ref(1)

  /////////////
  // GETTERS //
  /////////////

  const isPlaying = computed((): boolean => playing.value)
  const watch = computed(() => ({
    playing: playing.value,
    view: view.value,
    scale: scale.value,
  }))
  const getCurrentTime = computed((): number => currentTime.value)
  const getYScale = computed((): number => zoom.value.y)
  const getViewStart = computed((): number => view.value.start)
  const getViewEnd = computed((): number => view.value.end)
  const getViewDuration = computed((): number => view.value.end - view.value.start)
  const effectsChain = computed((): AudioEffectPayload[] => effects.value.map((e) => e.effect))
  const effectsChainForBackend = computed(() => serializeEffectChainForBackend(effectsChain.value))
  const getLevelsDb = computed((): number[] => levelsDb.value)

  /////////////
  // SETTERS //
  /////////////

  const setCurrentTime = (time: number): void => {
    currentTime.value = time
  }

  const play = async () => {
    if (!track.trackFilesExists) {
      alert.addAlert('There are no tracks to play', 'warning')
      return
    }

    setPlaying(true)
    startLevelPolling()
    await invoke('play')
  }

  const pause = async () => {
    setPlaying(false)
    stopLevelPolling(true)
    await invoke('pause')
  }

  const playPause = async () => {
    if (isPlaying.value) {
      await pause()
    } else {
      await play()
    }
  }

  const stop = async () => {
    await invoke('stop')
    currentTime.value = 0
    setPlaying(false)
    stopLevelPolling()
  }

  const setPlaying = (playingVal: boolean): void => {
    playing.value = playingVal
  }

  const togglePlaying = (): void => {
    playing.value = !playing.value
  }

  const setYScale = (y: number): void => {
    zoom.value.y = y
  }

  const clampViewRange = (start: number, end: number) => {
    const timelineDuration = Math.max(duration.value, 0)
    const minSpan = timelineDuration > 0 ? Math.min(0.5, timelineDuration) : 0.5

    if (timelineDuration <= 0) {
      view.value = { start: 0, end: 10 }
      return
    }

    let nextStart = Math.max(0, start)
    let nextEnd = Math.min(timelineDuration, end)
    let span = nextEnd - nextStart

    if (span < minSpan) {
      const mid = (nextStart + nextEnd) / 2
      nextStart = Math.max(0, mid - minSpan / 2)
      nextEnd = Math.min(timelineDuration, nextStart + minSpan)
      nextStart = Math.max(0, nextEnd - minSpan)
      span = nextEnd - nextStart
    }

    if (span > timelineDuration) {
      nextStart = 0
      nextEnd = timelineDuration
    }

    view.value = { start: nextStart, end: nextEnd }
  }

  const setViewRange = (start: number, end: number) => {
    clampViewRange(start, end)
  }

  const zoomView = (multiplier: number) => {
    const currentSpan = getViewDuration.value
    const timelineDuration = Math.max(duration.value, 0)
    if (timelineDuration <= 0 || currentSpan <= 0) return

    const anchor = Math.min(Math.max(clock.value, view.value.start), view.value.end)
    const nextSpan = currentSpan * multiplier
    const half = nextSpan / 2
    setViewRange(anchor - half, anchor + half)
  }

  const zoomIn = (axis?: 'x' | 'y' | 'both') => {
    axis = axis || 'x'
    if (axis === 'x' || axis === 'both') zoomView(0.8)
    if (axis === 'y' || axis === 'both') setYScale(getYScale.value + 1)
  }

  const zoomOut = (axis?: 'x' | 'y' | 'both') => {
    axis = axis || 'x'
    if (axis === 'x' || axis === 'both') zoomView(1.25)
    if (axis === 'y' || axis === 'both') setYScale(getYScale.value - 1)
  }

  const setDuration = async () => {
    await Tone.loaded()
    duration.value = toneMaster.duration
    setViewRange(0, duration.value)
  }

  const syncEffects = async () => {
    try {
      await invoke('set_effects_chain', { effects: effectsChainForBackend.value })
    } catch (error) {
      console.error('Failed to sync effects chain', error)
      alert.addAlert('Failed to sync effects chain', 'error')
    }
  }

  let syncTimer: ReturnType<typeof setTimeout> | undefined
  const scheduleSyncEffects = () => {
    if (syncTimer) clearTimeout(syncTimer)
    syncTimer = setTimeout(() => {
      void syncEffects()
    }, 150)
  }

  const addEffect = (effectType: AudioEffectType) => {
    const effect = createEffect(effectType)
    effects.value.push({ id: nextEffectId.value++, effect })
    void syncEffects()
  }

  const removeEffect = (id: number) => {
    const index = effects.value.findIndex((e) => e.id === id)
    if (index !== -1) effects.value.splice(index, 1)
    void syncEffects()
  }

  const moveEffect = (fromIndex: number, toIndex: number) => {
    if (fromIndex === toIndex) return
    if (fromIndex < 0 || fromIndex >= effects.value.length) return
    if (toIndex < 0 || toIndex >= effects.value.length) return

    const [item] = effects.value.splice(fromIndex, 1)
    effects.value.splice(toIndex, 0, item)
    void syncEffects()
  }

  const replaceEffects = (input: AudioEffectPayload[]) => {
    effects.value = effectChainFromPayload(input)
    nextEffectId.value = effects.value.reduce((max, item) => Math.max(max, item.id), 0) + 1
    void syncEffects()
  }

  const effectLabel = (effect: AudioEffectPayload) => getEffectLabel(effect)

  type zoomType = 'increment' | 'decrement'

  const zoomX = (direction: zoomType) => {
    if (direction === 'increment') zoomIn('x')
    else zoomOut('x')
  }

  const zoomY = (direction: zoomType) => {
    if (direction === 'increment' && zoom.value.y < 20) {
      zoom.value.y++
    } else if (direction === 'decrement' && zoom.value.y > 1) {
      zoom.value.y--
    }
  }

  const setClock = (time: number) => {
    clock.value = time
  }

  const seek = async (time: number) => {
    await invoke('seek', { position: time })
  }

  const setLevelsDb = (levels: number[]) => {
    if (levels.length === 0) {
      levelsDb.value = levelsDb.value.length ? levelsDb.value : [-60, -60]
      return
    }
    levelsDb.value = levels
  }

  let levelsTimer: ReturnType<typeof setInterval> | undefined
  const refreshLevels = async () => {
    console.log('refreshLevels')
    try {
      const levels = await invoke<number[]>('get_levels_db')
      if (Array.isArray(levels)) {
        setLevelsDb(levels)
      }
    } catch (error) {
      console.error('Failed to refresh levels', error)
    }
  }

  const startLevelPolling = () => {
    if (levelsTimer) return
    void refreshLevels()
    levelsTimer = setInterval(() => {
      void refreshLevels()
    }, 60)
  }

  const stopLevelPolling = (paused = false) => {
    if (levelsTimer) {
      clearInterval(levelsTimer)
      levelsTimer = undefined
    }
    if (!paused) {
      setLevelsDb(levelsDb.value.map(() => -60))
    }
  }

  vueWatch(
    effects,
    () => {
      scheduleSyncEffects()
    },
    { deep: true, immediate: true },
  )

  return {
    scale,
    zoom,
    view,
    effects,
    effectsChain,
    effectsChainForBackend,
    duration,
    watch,
    isPlaying,
    getCurrentTime,
    getYScale,
    getViewStart,
    getViewEnd,
    getViewDuration,
    getLevelsDb,
    clock,
    play,
    pause,
    playPause,
    stop,
    setYScale,
    setViewRange,
    zoomIn,
    zoomOut,
    setPlaying,
    setCurrentTime,
    togglePlaying,
    setDuration,
    addEffect,
    removeEffect,
    moveEffect,
    replaceEffects,
    effectLabel,
    zoomX,
    zoomY,
    setClock,
    seek,
    syncEffects,
    scheduleSyncEffects,
    refreshLevels,
    startLevelPolling,
    stopLevelPolling,
  }
})
