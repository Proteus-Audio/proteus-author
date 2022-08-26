<script setup lang="ts">
import Hello from "./components/Hello.vue";
import { ipcRenderer } from "./electron";
import { ref } from "vue";

ipcRenderer.send("message", "Hello from App.vue!");
const fileName = ref("");

const hello = (): void => {
  ipcRenderer.send("message", "Hello again!");
};

const open = async (): Promise<void> => {
  // console.log(await ipcRenderer.send("openFile", "main"));
  
  fileName.value = await ipcRenderer.invoke("openFile", "main");

};
</script>

<template>
  <div id="app">
    <Hello />
    <button @click="hello">Hello!</button><br />
    {{ fileName }}<br />
    <button @click="open">Open File</button>
  </div>
</template>
