<template>
  <div class="fx-icon" @click.stop="toggleEdit">
    <el-icon class="inner" style="vertical-align: middle" :size="20" color="#ffffff">
      <SuitcaseLine v-if="type === 'Compressor'" />
      <Phone v-if="type === 'Reverb'" />
      <!-- <QuestionFilled v-if="type " /> -->
    </el-icon>
    <el-dialog v-model="editOpen" width="calc(100% - 4em)">
      <EffectDialogCompressor :effectIndex="index" v-if="type === 'Compressor'" />
      <EffectDialogReverb :effectIndex="index" v-if="type === 'Reverb'" />
      <div>
        <el-button :icon="Close" @click="toggleEdit">Close</el-button>
        <el-button :icon="Delete" @click="removeEffect">Remove Effect</el-button>
      </div>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { Effect } from "../../typings/effects";
import { SuitcaseLine, Phone, Close, Delete } from "@element-plus/icons-vue";
import { toneMaster } from "../../public/toneMaster";
import EffectDialogCompressor from "./EffectDialogCompressor.vue";
import EffectDialogReverb from "./EffectDialogReverb.vue";
import { useAudioStore } from "../../stores/audio";
import { Compressor, Reverb } from 'tone';
import { CompressorSettings, ReverbSettings } from '../../public/effects';

interface Props {
  id: number;
  type: Effect;
  index: number;
}

const emit = defineEmits(["remove"]);
const audio = useAudioStore();

const props = defineProps<Props>();

const editOpen = ref(false);

const toggleEdit = () => {
  editOpen.value = !editOpen.value;
};

const removeEffect = () => {
  console.log(`Deleting #${props.id}`)
  audio.removeEffect(props.id);
  // toneMaster.removeEffect(props.type);
  // emit("remove");
};

onMounted(() => {
  const instance = audio.effects[props.index];
  console.log(instance, props.index, props.id, props.type)
  if (instance.effect instanceof CompressorSettings) {
    const { threshold, ratio, knee, attack, release } = instance.effect;
    toneMaster.addEffect(new Compressor({ threshold, ratio, knee, attack, release }));
  } else if (instance.effect instanceof ReverbSettings) {
    const { decay, wet, preDelay } = instance.effect;
    toneMaster.addEffect(new Reverb({ decay, wet, preDelay }));
  }
  console.log(toneMaster.effects);
});

onUnmounted(() => {
  console.log('destroy', props.id, props.index);
  toneMaster.removeEffect(props.index);
});
</script>

<style lang="scss" scoped>
.fx-icon {
  width: 100%;
  height: 100%;
  background-color: rgb(69, 69, 69);
  margin-top: 0em;
  border-radius: 0.5em;
  padding: 1em;
  color: white;
  display: block;
  overflow: hidden;
  transition: height 0.3s, margin 0.3s;

  display: grid;
  align-items: center;
  align-content: center;
  text-align: center;

  &:hover {
    height: 110%;
    margin-top: -2.5%;
  }

  .inner {
    margin: auto;
  }
}
</style>
