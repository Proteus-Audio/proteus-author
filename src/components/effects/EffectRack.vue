<template>
  <el-dropdown id="effect-rack" :class="rackClass" trigger="click">
    <div>
      <div class="no-effects" v-if="noEffects">There are no effects, click to add one</div>
      <div class="effects-list" v-else>
        <EffectMini
          class="effect"
          v-for="(effect, i) in effects"
          :key="effect.id"
          :item="effect"
          :index="i"
        />
      </div>
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
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(7rem, 1fr));
    gap: 1em;
    height: 100%;
  }
}
</style>
