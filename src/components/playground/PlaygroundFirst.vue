<template>
  <div id="hello">
    <div class="large">
      <img :src="imageSrc" />
    </div>
    <el-button label="New Header" @click="getImage" :icon="IceCream" class="">New Header</el-button>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { invoke } from "@tauri-apps/api/core";

import { IceCream } from '@element-plus/icons-vue'

const fileName = ref('')
const filePath = ref('')
const fileSrc = ref('')
const directory = ref('/')

const imageSrc = computed(() =>
  // fileSrc.value !== "" ? fileSrc.value : "./images/sevs_nevess.png"
  filePath.value !== '' ? `file://${filePath.value}` : './images/sevs_nevess.png',
)

const getImage = async (): Promise<void> => {
  // const file = await invoke('openFile', {directory: directory.value}) as File
  // if (file === 'canceled') return
  // if (!/image/i.test(file.type)) {
  //   return
  // }
  // fileName.value = file.fileName
  // filePath.value = file.filePath
  // fileSrc.value = file.src
}
</script>

<style lang="scss" scoped>
#hello {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin: 60px 0;
  .small {
    max-width: 150px;
    margin: auto;
  }
  .large {
    max-width: 550px;
    margin: auto;
  }
}
img {
  width: 100%;
}
</style>
