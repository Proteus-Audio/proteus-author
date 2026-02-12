<template>
  <div class="fx-icon" @click.stop="toggleEdit">
    <div class="fx-indicator">
      <AnalogIndicator size="small" :state="true" :color="enabled ? 'green' : 'red'" />
    </div>
    <div class="fx-label">{{ label }}</div>
    <el-dialog
      v-model="editOpen"
      width="calc(100% - 4em)"
      style="height: fit-content"
      align-center
      :append-to-body="true"
      :close-on-click-modal="true"
    >
      <div class="dialog-body" @click.stop>
        <EffectDialog :effectIndex="index" />
        <div class="dialog-actions">
          <el-button :icon="Close" @click="toggleEdit">Close</el-button>
          <el-button :icon="Delete" @click="removeEffect">Remove Effect</el-button>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { Close, Delete } from '@element-plus/icons-vue'
import EffectDialog from './EffectsDialog.vue'
import { AnalogIndicator } from '../analog'
import { useAudioStore } from '../../stores/audio'
import type { EffectChainItem } from '../../assets/effects'
import { EffectSettings } from '../../typings/effects'

interface Props {
  item: EffectChainItem
  index: number
}

const audio = useAudioStore()
const props = defineProps<Props>()

const editOpen = ref(false)

const label = computed(() => audio.effectLabel(props.item.effect))
const toggleEdit = () => {
  editOpen.value = !editOpen.value
}

const effect = computed((): EffectSettings | undefined => {
  return undefined
  return Object.values(props.item.effect)[0]
})

const enabled = computed({
  get() {
    return effect.value?.enabled ?? false
  },
  set(value) {
    if (effect.value) {
      effect.value.enabled = value
    }
  },
})

const removeEffect = () => {
  audio.removeEffect(props.item.id)
}
</script>

<style lang="scss" scoped>
.fx-icon {
  position: relative;
  width: max-content;
  min-width: max-content;
  height: 100%;
  background-color: rgb(69, 69, 69);
  margin-top: 0em;
  border-radius: 0.5em;
  padding: 0.75em 2em;
  color: white;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  /* grid-template-rows: 1fr auto; */
  gap: 0.5em;
  overflow: hidden;
  transition:
    height 0.3s,
    margin 0.3s;
  cursor: grab;

  &:hover {
    opacity: 0.85;
  }

  .fx-indicator {
    position: absolute;
    top: 0.75rem;
    left: 0.75rem;
  }
}

.fx-label {
  font-weight: 600;
  text-align: center;
  display: grid;
  align-items: center;
  white-space: nowrap;
}

.dialog-actions {
  display: grid;
  grid-template-columns: auto auto;
  justify-content: end;
  gap: 0.75em;
  margin-top: 1em;
}
</style>
