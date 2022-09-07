<template>
  <div class="fx-icon" @click.stop="doSomething">
    <el-icon class="inner" style="vertical-align: middle" :size="20" color="#ffffff">
        <SuitcaseLine v-if="type === 'Compressor'" />
        <Phone v-if="type === 'Reverb'" />
      <!-- <QuestionFilled v-if="type " /> -->
    </el-icon>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import Effect from "../../typings/effects";
import {
  SuitcaseLine,
  Phone,
  QuestionFilled,
  Refresh,
  ZoomOut,
  ZoomIn,
} from "@element-plus/icons-vue";
import { toneMaster } from '../../public/toneMaster';

interface Props {
  type: Effect;
}

const emit = defineEmits(["remove"]);

const props = defineProps<Props>();

const doSomething = () => {
    toneMaster.removeEffect(props.type);
    emit('remove');
};
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
