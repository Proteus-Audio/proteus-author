import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { defineStore } from 'pinia'
import { computed, type Ref, ref, watch as vueWatch } from 'vue'
import {
  createEffect,
  type EffectChainItem,
  effectChainFromPayload,
  effectSupportsFrequencyResponse,
  effectSupportsSpectralOverlay,
  getEffectLabel,
  getEffectType,
  serializeEffectChainForBackend,
} from '../utils/effects'
import type {
  AudioEffectPayload,
  AudioEffectType,
  EffectBandSnapshot,
  EffectLevelSnapshot,
  FilterResponseCurve,
} from '../typings/effects'
import { useAlertStore } from './alerts'
import { useTrackStore } from './track'

export const useAudioStore = defineStore('prot', () => {
  const EFFECT_RESPONSE_POINTS = 128
  const EFFECT_LEVEL_REFRESH_HZ = 30
  const EFFECT_SPECTRAL_REFRESH_HZ = 15

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
  const followMode = ref(true)
  const shufflePointToolMode = ref(false)
  const effects = ref([] as EffectChainItem[])
  const clock: Ref<number> = ref(0.0)
  const levelsDb = ref([-60, -60] as number[])
  const activeEffectDialogId = ref<number | null>(null)
  const effectLevelSnapshots = ref<EffectLevelSnapshot[] | null>(null)
  const effectBandSnapshots = ref<(EffectBandSnapshot | null)[] | null>(null)
  const effectResponseCache = ref<Record<string, Array<FilterResponseCurve | null>>>({})
  const effectVisualizationGeneration = ref(0)
  const effectLevelMeteringEnabled = ref(false)
  const effectSpectralAnalysisEnabled = ref(false)

  const nextEffectId = ref(1)

  /////////////
  // GETTERS //
  /////////////

  const isPlaying = computed((): boolean => playing.value)
  const watch = computed(() => ({
    playing: playing.value,
    view: view.value,
    followMode: followMode.value,
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
  const activeEffectIndex = computed(() => {
    if (activeEffectDialogId.value == null) return -1
    return effects.value.findIndex((item) => item.id === activeEffectDialogId.value)
  })
  const activeEffect = computed(() => {
    if (activeEffectDialogId.value == null) return undefined
    return effects.value.find((item) => item.id === activeEffectDialogId.value)
  })
  const activeEffectPayload = computed(() => activeEffect.value?.effect)
  const activeEffectType = computed(() =>
    activeEffectPayload.value ? getEffectType(activeEffectPayload.value) : undefined,
  )
  const activeEffectResponseCacheKey = computed(() => {
    return `${effectVisualizationGeneration.value}:${JSON.stringify(effectsChainForBackend.value)}:${EFFECT_RESPONSE_POINTS}`
  })
  const activeEffectResponseCurve = computed(() => {
    const index = activeEffectIndex.value
    if (index < 0) return null
    const curves = effectResponseCache.value[activeEffectResponseCacheKey.value]
    if (!curves) return null
    return curves[index] ?? null
  })
  const activeEffectLevelSnapshot = computed(() => {
    const index = activeEffectIndex.value
    if (index < 0 || !effectLevelSnapshots.value) return null
    return effectLevelSnapshots.value[index] ?? null
  })
  const activeEffectBandSnapshot = computed(() => {
    const index = activeEffectIndex.value
    if (index < 0 || !effectBandSnapshots.value) return null
    return effectBandSnapshots.value[index] ?? null
  })

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
      const nextStart = Math.max(0, start)
      let nextEnd = Math.max(end, nextStart + minSpan)
      if (nextEnd - nextStart < minSpan) {
        nextEnd = nextStart + minSpan
      }
      view.value = { start: nextStart, end: nextEnd }
      return
    }

    const requestedSpan = Math.max(end - start, minSpan)
    const span = Math.min(requestedSpan, timelineDuration)
    const center = (start + end) / 2

    let nextStart = center - span / 2
    let nextEnd = center + span / 2

    if (nextStart < 0) {
      nextEnd -= nextStart
      nextStart = 0
    }

    if (nextEnd > timelineDuration) {
      const overflow = nextEnd - timelineDuration
      nextStart -= overflow
      nextEnd = timelineDuration
    }

    if (nextStart < 0) {
      nextStart = 0
    }

    view.value = { start: nextStart, end: nextEnd }
  }

  const setViewRange = (start: number, end: number) => {
    clampViewRange(start, end)
  }

  const setFollowMode = (enabled: boolean) => {
    followMode.value = enabled
  }

  const toggleFollowMode = () => {
    followMode.value = !followMode.value
  }

  const setShufflePointToolMode = (enabled: boolean) => {
    shufflePointToolMode.value = enabled
  }

  const toggleShufflePointToolMode = () => {
    setShufflePointToolMode(!shufflePointToolMode.value)
  }

  const panViewByFraction = (fraction: number) => {
    if (!Number.isFinite(fraction) || fraction === 0) return
    const span = getViewDuration.value
    if (span <= 0) return
    const shiftSeconds = span * fraction
    setViewRange(view.value.start + shiftSeconds, view.value.end + shiftSeconds)
  }

  const panViewLeft = (fraction = 0.2) => {
    panViewByFraction(-Math.abs(fraction))
  }

  const panViewRight = (fraction = 0.2) => {
    panViewByFraction(Math.abs(fraction))
  }

  const zoomView = (multiplier: number) => {
    const currentSpan = getViewDuration.value
    if (currentSpan <= 0) return

    const fallbackCenter = (view.value.start + view.value.end) / 2
    const anchor = Number.isFinite(clock.value) ? Math.max(clock.value, 0) : fallbackCenter
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
    let nextDuration = 0
    try {
      nextDuration = await invoke<number>('get_duration')
    } catch {
      nextDuration = 0
    }

    duration.value = nextDuration

    if (duration.value > 0) {
      const currentSpan = getViewDuration.value
      if (
        currentSpan <= 0 ||
        view.value.start < 0 ||
        view.value.end > duration.value ||
        currentSpan > duration.value
      ) {
        setViewRange(0, duration.value)
      }
      return
    }

    if (getViewDuration.value <= 0) {
      setViewRange(0, 10)
    }
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
    if (activeEffectDialogId.value === id) {
      activeEffectDialogId.value = null
    }
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

  const effectIndexFromId = (effectId: number) => {
    return effects.value.findIndex((item) => item.id === effectId)
  }

  const effectLevelSnapshotForId = (effectId: number): EffectLevelSnapshot | null => {
    const index = effectIndexFromId(effectId)
    if (index < 0 || !effectLevelSnapshots.value) return null
    return effectLevelSnapshots.value[index] ?? null
  }

  const effectBandSnapshotForId = (effectId: number): EffectBandSnapshot | null => {
    const index = effectIndexFromId(effectId)
    if (index < 0 || !effectBandSnapshots.value) return null
    return effectBandSnapshots.value[index] ?? null
  }

  const effectResponseCurveForId = (effectId: number): FilterResponseCurve | null => {
    const index = effectIndexFromId(effectId)
    if (index < 0) return null
    const curves = effectResponseCache.value[activeEffectResponseCacheKey.value]
    if (!curves) return null
    return curves[index] ?? null
  }

  const clearEffectVisualizationSnapshots = () => {
    effectLevelSnapshots.value = null
    effectBandSnapshots.value = null
  }

  let effectLevelsTimer: ReturnType<typeof setInterval> | undefined
  let effectSpectralTimer: ReturnType<typeof setInterval> | undefined
  let effectResponseTimer: ReturnType<typeof setTimeout> | undefined
  let effectPlayerChangedUnlisten: UnlistenFn | undefined

  const ensureEffectVisualizationListener = async () => {
    if (effectPlayerChangedUnlisten) return
    effectPlayerChangedUnlisten = await listen('PLAYER_CHANGED', () => {
      effectVisualizationGeneration.value += 1
      clearEffectVisualizationSnapshots()
      void syncEffectVisualizationState(true)
    })
  }

  const setEffectLevelMetering = async (enabled: boolean, force = false) => {
    if (!force && effectLevelMeteringEnabled.value === enabled) return
    effectLevelMeteringEnabled.value = enabled

    try {
      await invoke('set_effect_level_metering', {
        enabled,
        refreshHz: EFFECT_LEVEL_REFRESH_HZ,
      })
    } catch (error) {
      console.error('Failed to configure effect level metering', error)
    }
  }

  const setEffectSpectralAnalysis = async (enabled: boolean, force = false) => {
    if (!force && effectSpectralAnalysisEnabled.value === enabled) return
    effectSpectralAnalysisEnabled.value = enabled

    try {
      await invoke('set_effect_spectral_analysis', {
        enabled,
        refreshHz: EFFECT_SPECTRAL_REFRESH_HZ,
      })
    } catch (error) {
      console.error('Failed to configure effect spectral analysis', error)
    }
  }

  const refreshEffectLevels = async () => {
    if (
      !effectLevelMeteringEnabled.value ||
      !isPlaying.value ||
      activeEffectDialogId.value == null
    ) {
      effectLevelSnapshots.value = null
      return
    }

    try {
      effectLevelSnapshots.value = await invoke<EffectLevelSnapshot[] | null>(
        'get_effect_levels_audible',
      )
    } catch (error) {
      console.error('Failed to refresh effect levels', error)
    }
  }

  const refreshEffectBandSnapshots = async () => {
    if (
      !effectSpectralAnalysisEnabled.value ||
      !isPlaying.value ||
      activeEffectDialogId.value == null
    ) {
      effectBandSnapshots.value = null
      return
    }

    try {
      effectBandSnapshots.value = await invoke<(EffectBandSnapshot | null)[] | null>(
        'get_effect_band_levels_audible',
      )
    } catch (error) {
      console.error('Failed to refresh effect band snapshots', error)
    }
  }

  const refreshEffectResponseCurves = async () => {
    if (
      activeEffectDialogId.value == null ||
      !activeEffectType.value ||
      !effectSupportsFrequencyResponse(activeEffectType.value)
    ) {
      return
    }

    const key = activeEffectResponseCacheKey.value
    if (effectResponseCache.value[key]) return

    try {
      const responses = await invoke<(FilterResponseCurve | null)[]>(
        'get_effect_frequency_responses',
        {
          numPoints: EFFECT_RESPONSE_POINTS,
        },
      )
      effectResponseCache.value = {
        ...effectResponseCache.value,
        [key]: responses,
      }
    } catch (error) {
      console.error('Failed to refresh effect response curves', error)
    }
  }

  const scheduleEffectResponseRefresh = () => {
    if (effectResponseTimer) clearTimeout(effectResponseTimer)
    effectResponseTimer = setTimeout(() => {
      void refreshEffectResponseCurves()
    }, 120)
  }

  const startEffectLevelPolling = () => {
    if (effectLevelsTimer || activeEffectDialogId.value == null || !isPlaying.value) return
    void refreshEffectLevels()
    effectLevelsTimer = setInterval(
      () => {
        void refreshEffectLevels()
      },
      Math.max(1000 / EFFECT_LEVEL_REFRESH_HZ, 16),
    )
  }

  const stopEffectLevelPolling = (reset = true) => {
    if (effectLevelsTimer) {
      clearInterval(effectLevelsTimer)
      effectLevelsTimer = undefined
    }
    if (reset) {
      effectLevelSnapshots.value = null
    }
  }

  const startEffectSpectralPolling = () => {
    if (effectSpectralTimer || activeEffectDialogId.value == null || !isPlaying.value) return
    void refreshEffectBandSnapshots()
    effectSpectralTimer = setInterval(
      () => {
        void refreshEffectBandSnapshots()
      },
      Math.max(1000 / EFFECT_SPECTRAL_REFRESH_HZ, 30),
    )
  }

  const stopEffectSpectralPolling = (reset = true) => {
    if (effectSpectralTimer) {
      clearInterval(effectSpectralTimer)
      effectSpectralTimer = undefined
    }
    if (reset) {
      effectBandSnapshots.value = null
    }
  }

  const syncEffectVisualizationState = async (force = false) => {
    await ensureEffectVisualizationListener()

    const type = activeEffectType.value
    const hasActiveDialog = activeEffectDialogId.value != null && activeEffectIndex.value >= 0

    if (!hasActiveDialog || !type) {
      stopEffectLevelPolling()
      stopEffectSpectralPolling()
      clearEffectVisualizationSnapshots()
      await setEffectSpectralAnalysis(false, force)
      await setEffectLevelMetering(false, force)
      return
    }

    await setEffectLevelMetering(true, force)

    if (effectSupportsFrequencyResponse(type)) {
      scheduleEffectResponseRefresh()
    }

    if (!isPlaying.value) {
      stopEffectLevelPolling()
      stopEffectSpectralPolling()
      await setEffectSpectralAnalysis(false, force)
      effectBandSnapshots.value = null
      return
    }

    startEffectLevelPolling()

    const spectralEnabled = effectSupportsSpectralOverlay(type)
    await setEffectSpectralAnalysis(spectralEnabled, force)
    if (spectralEnabled) {
      startEffectSpectralPolling()
    } else {
      stopEffectSpectralPolling()
    }
  }

  const openEffectDialog = (effectId: number) => {
    activeEffectDialogId.value = effectId
    clearEffectVisualizationSnapshots()
    void syncEffectVisualizationState(true)
  }

  const closeEffectDialog = (effectId?: number) => {
    if (effectId != null && activeEffectDialogId.value !== effectId) return
    activeEffectDialogId.value = null
    void syncEffectVisualizationState(true)
  }

  let levelsTimer: ReturnType<typeof setInterval> | undefined
  const refreshLevels = async () => {
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
      clearEffectVisualizationSnapshots()
      if (activeEffectDialogId.value != null && activeEffectIndex.value < 0) {
        activeEffectDialogId.value = null
      }
      if (activeEffectDialogId.value != null) {
        void syncEffectVisualizationState()
      }
    },
    // Avoid startup-time no-op sync when effect chain is still empty.
    // Runtime edits and loaded-project replacements still sync via this watcher and explicit calls.
    { deep: true },
  )

  vueWatch([activeEffectDialogId, isPlaying], () => {
    void syncEffectVisualizationState()
  })

  return {
    scale,
    zoom,
    view,
    followMode,
    shufflePointToolMode,
    effects,
    effectsChain,
    effectsChainForBackend,
    duration,
    watch,
    isPlaying,
    activeEffectDialogId,
    activeEffectIndex,
    activeEffectType,
    activeEffectResponseCurve,
    activeEffectLevelSnapshot,
    activeEffectBandSnapshot,
    effectLevelMeteringEnabled,
    effectSpectralAnalysisEnabled,
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
    setFollowMode,
    toggleFollowMode,
    setShufflePointToolMode,
    toggleShufflePointToolMode,
    panViewByFraction,
    panViewLeft,
    panViewRight,
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
    effectLevelSnapshotForId,
    effectBandSnapshotForId,
    effectResponseCurveForId,
    openEffectDialog,
    closeEffectDialog,
    syncEffectVisualizationState,
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
