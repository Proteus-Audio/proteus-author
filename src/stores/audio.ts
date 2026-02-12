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
  const zoom = ref({ y: 1, x: 10 })
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
    zoom: zoom.value,
    scale: scale.value,
  }))
  const getCurrentTime = computed((): number => currentTime.value)
  const getXScale = computed((): number => zoom.value.x)
  const getYScale = computed((): number => zoom.value.y)
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

  const setXScale = (x: number): void => {
    zoom.value.x = x
  }

  const setYScale = (y: number): void => {
    zoom.value.y = y
  }

  const zoomIn = (axis?: 'x' | 'y' | 'both', degree?: number) => {
    axis = axis || 'x'
    const amount = degree ? degree / 100 : 1
    if (axis === 'x' || axis === 'both') setXScale(getXScale.value + 1 * amount)
    if (axis === 'y' || axis === 'both') setYScale(getXScale.value + 1 * amount)
  }

  const zoomOut = (axis?: 'x' | 'y' | 'both', degree?: number) => {
    axis = axis || 'x'
    const amount = degree ? degree / 100 : 1
    if (axis === 'x' || axis === 'both') setXScale(getXScale.value - 1 * amount)
    if (axis === 'y' || axis === 'both') setYScale(getXScale.value - 1 * amount)
  }

  const setDuration = async () => {
    await Tone.loaded()
    duration.value = toneMaster.duration
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
    if (direction === 'increment' && zoom.value.x < 20) {
      zoom.value.x++
    } else if (direction === 'decrement' && zoom.value.x > 1) {
      zoom.value.x--
    }
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
    effects,
    effectsChain,
    effectsChainForBackend,
    duration,
    watch,
    isPlaying,
    getCurrentTime,
    getXScale,
    getYScale,
    getLevelsDb,
    clock,
    play,
    pause,
    playPause,
    stop,
    setXScale,
    setYScale,
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
