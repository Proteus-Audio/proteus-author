<template>
  <div id="effect-rack" :class="`${rackClass}`" @click="addEffect">
    <div class="no-effects" v-if="noEffects">There are no effects, click to add one</div>
    <div class="effects-list" v-else>
      <EffectMini
        class="effect"
        v-for="(effect, i) in effects"
        @remove="() => remove(effect, i)"
        :key="i"
        :type="effect"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Compressor, Reverb } from "tone";
import { computed, ref } from "vue";
import { toneMaster } from "../../public/toneMaster";
import { useAudioStore } from "../../stores/audio";
import { Effect } from "../../typings/effects";
import EffectMini from "./EffectMini.vue";

const effects = ref([] as Effect[]);
const effectsToAdd: Effect[] = ["Reverb", "Compressor"];

const noEffects = computed(() => effects.value.length <= 0);

const rackClass = computed(() => (noEffects.value ? "empty" : "full"));

const audio = useAudioStore();

const addEffect = () => {
  const toAdd = effectsToAdd.shift();
  if (toAdd) {
    toAdd === "Compressor"
      ? toneMaster.addEffect(
          new Compressor({
            threshold: audio.compressor.threshold,
            ratio: audio.compressor.ratio,
            knee: audio.compressor.knee,
            attack: audio.compressor.attack,
            release: audio.compressor.release
          })
        )
      : toneMaster.addEffect(
          new Reverb({
            decay: audio.reverb.decay,
            wet: audio.reverb.mix,
            preDelay: audio.reverb.preDelay,
          })
        );
    effects.value.push(toAdd);
  }
};

const remove = (effect: Effect, index: number) => {
  effects.value.splice(index, 1);
  effectsToAdd.push(effect);
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
