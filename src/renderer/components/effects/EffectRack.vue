<template>
  <div id="effect-rack" :class="`${rackClass}`" @click="addEffect">
    <div class="no-effects" v-if="noEffects">There are no effects, click to add one</div>
    <div class="effects-list" v-else>
      <EffectMini class="effect" v-for="(effect, i) in effects" :key="i" :type="effect" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import Effect from "../../typings/effects";
import EffectMini from "./EffectMini.vue";

const effects = ref([] as Effect[]);
const effectsToAdd: Effect[] = ["compressor", "reverb"];

const noEffects = computed(() => effects.value.length <= 0);

const rackClass = computed(() => (noEffects.value ? "empty" : "full"));

const addEffect = () => {
  const toAdd = effectsToAdd.shift();
  if (toAdd) effects.value.push(toAdd);
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
    .effect {
    }
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
