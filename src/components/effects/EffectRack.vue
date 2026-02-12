<template>
  <el-dropdown id="effect-rack" :class="rackClass" trigger="click">
    <div>
      <div class="no-effects" v-if="noEffects">There are no effects, click to add one</div>
      <Draggable
        v-else
        class="effects-list"
        :list="effects"
        item-key="id"
        ghost-class="effect-ghost"
        chosen-class="effect-chosen"
        drag-class="effect-drag"
        :animation="160"
      >
        <template #item="{ element, index }">
          <div class="effect-wrapper">
            <EffectMini class="effect" :item="element" :index="index" />
          </div>
        </template>
      </Draggable>
    </div>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item v-for="type in effectTypes" :key="type" @click="() => addEffect(type)">
          {{ effectTypeLabels[type] }}
        </el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Draggable from 'vuedraggable'
import { useAudioStore } from '../../stores/audio'
import EffectMini from './EffectMini.vue'
import { effectTypes, effectTypeLabels } from '../../assets/effects'
import type { AudioEffectType } from '../../typings/effects'

const audio = useAudioStore()

const effects = computed(() => audio.effects)
const noEffects = computed(() => effects.value.length <= 0)
const rackClass = computed(() => (noEffects.value ? 'empty' : 'full'))

const addEffect = (toAdd: AudioEffectType) => {
  audio.addEffect(toAdd)
}
</script>

<style lang="scss" scoped>
#effect-rack {
  width: 100%;
  padding: 1em;
  background-color: rgb(229 229 229);
  border-top: 2px solid grey;
  position: fixed;
  bottom: 0;
  left: 0;
  height: 5em;
  height: var(--effect-rack-height);
  transition: height 0.3s;
  align-content: center;
  cursor: pointer;
  z-index: 20;

  &.empty {
    display: grid;
  }
  &.full {
    display: block;
  }

  .no-effects {
    text-align: center;
    text-transform: uppercase;
    color: grey;
    font-size: 0.8em;
  }

  .effects-list {
    display: flex;
    flex-wrap: nowrap;
    align-items: stretch;
    gap: 1em;
    height: 100%;
    width: 100%;
    overflow-x: auto;
    overflow-y: hidden;
    padding-bottom: 0.25em;
    scrollbar-gutter: stable;
  }

  .effect-wrapper {
    flex: 0 0 auto;
    width: max-content;
    transition:
      transform 0.15s ease-out,
      box-shadow 0.15s ease-out;
  }

  .effect-ghost {
    opacity: 0.4;
  }

  .effect-chosen {
    opacity: 0.85;
    transform: scale(0.98);
  }

  .effect-drag {
    opacity: 0.9;
    box-shadow: 0 12px 20px rgba(0, 0, 0, 0.35);
  }
}
</style>
