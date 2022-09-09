<template>
  <el-dropdown id="effect-rack" trigger="click">
    <div :class="`${rackClass}`">
      <div class="no-effects" v-if="noEffects">There are no effects, click to add one</div>
      <div class="effects-list" v-else>
        <EffectMini
          class="effect"
          v-for="(effect, i) in effects"
          :key="effect.id"
          :type="effect.type"
          :id="effect.id"
          :index="i"
        />
      </div>
    </div>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item @click="() => addEffect('Reverb')">Reverb</el-dropdown-item>
        <el-dropdown-item @click="() => addEffect('Compressor')">Compression</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useAudioStore } from "../../stores/audio";
import EffectMini from "./EffectMini.vue";

const effects = computed(() => audio.effects)
const noEffects = computed(() => effects.value.length <= 0);
const rackClass = computed(() => (noEffects.value ? "empty" : "full"));

const audio = useAudioStore();

const addEffect = (toAdd: "Reverb" | "Compressor") => {
  if (toAdd === "Compressor") audio.addEffect("Compressor");
  else if (toAdd === "Reverb") audio.addEffect("Reverb");
};
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
  transition: height 0.3s;
  align-content: center;
  cursor: pointer;
  z-index: 20;

  &.empty {
    // background-color: green;
    display: grid;
  }
  &.full {
    // grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
    display: block;
  }

  &:hover {
    height: 6em;
  }
  .no-effects {
    text-align: center;
    text-transform: uppercase;
    // font-weight: bold;
    color: grey;
    font-size: 0.8em;
  }

  .effects-list {
    display: grid;
    grid-template-columns: 7em 7em 7em 7em;
    gap: 1em;
    height: 100%;
  }
}
</style>
