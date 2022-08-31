<template>
  <input
    type="text"
    class="track-name"
    placeholder="Click to Add Name"
    :value="modelValue"
    @input="$emit(update:modelValue, $event.target.value)"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue';

interface Props {
  placeholder?: string;
  modelValue: string;
}

// ref

const props = defineProps<Props>();

const updateValue = (event) => {
    
}

const hide = document.getElementById("hide");
const txt = document.getElementById("txt");
resize();
txt.addEventListener("input", resize);

function resize() {
  hide.textContent = txt.value;
  txt.style.width = hide.offsetWidth + "px";
}
</script>

<style scoped></style>
