<template>
  <div class="fx-icon" @click.stop="toggleEdit">
    <div class="fx-label">{{ label }}</div>
    <el-dialog
      v-model="editOpen"
      width="calc(100% - 4em)"
      align-center
      :append-to-body="true"
      :close-on-click-modal="false"
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
import { useAudioStore } from '../../stores/audio'
import type { EffectChainItem } from '../../assets/effects'

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

const removeEffect = () => {
  audio.removeEffect(props.item.id)
}

</script>

<style lang="scss" scoped>
.fx-icon {
  width: 100%;
  height: 100%;
  background-color: rgb(69, 69, 69);
  margin-top: 0em;
  border-radius: 0.5em;
  padding: 0.75em;
  color: white;
  display: grid;
  grid-template-rows: 1fr auto;
  gap: 0.5em;
  overflow: hidden;
  transition:
    height 0.3s,
    margin 0.3s;
  cursor: grab;

  &:hover {
    height: 110%;
    margin-top: -2.5%;
  }
}

.fx-label {
  font-weight: 600;
  text-align: center;
  display: grid;
  align-items: center;
}

.dialog-actions {
  display: grid;
  grid-template-columns: auto auto;
  justify-content: end;
  gap: 0.75em;
  margin-top: 1em;
}
</style>
