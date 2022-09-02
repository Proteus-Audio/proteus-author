<template>
  <div class="auto-size">
    <input
      ref="textInput"
      type="text"
      class="input"
      :style="`width: ${width}px;`"
      placeholder="Click to Add Name"
      :value="modelValue"
      @input="updateValue"
    />
    <div ref="sizer" class="sizer">{{ sizerText }}</div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUpdated, ref } from "vue";

interface Props {
  placeholder?: string;
  modelValue: string;
}

const emit = defineEmits(["update:modelValue"]);

const props = defineProps<Props>();

const updateValue = (event: Event) => {
  emit("update:modelValue", (event.target as HTMLTextAreaElement).value);
};

const textInput = ref(null as HTMLElement | null);
const sizer = ref(null as HTMLElement | null);
const width = ref(0);

const sizerText = computed(() => props.modelValue !== "" ? props.modelValue : (props.placeholder || ""))

const resize = () => {
  width.value = (sizer.value?.offsetWidth) || width.value;
//   txt.style.width = hide.offsetWidth + "px";
}

onMounted(() => {
  console.log(sizer.value);
  resize();
});

onUpdated(() => {
    resize();
})



// const hide = document.getElementById("hide");
// const txt = document.getElementById("txt");
// resize();
// txt.addEventListener("input", resize);

// function resize() {
// }
</script>

<style lang="scss" scoped>
.auto-size {
  .input {
    font-family: inherit;
    font-size: 1em;
    background: transparent;
    border: none;
    max-width: 100%;
    min-width: 10px;
    width: fit-content;
    margin-bottom: 1em;
    padding: 0;

    &:focus,
    &:focus-visible {
      border: none;
      outline: none;
    }
  }

  .sizer {
    position: absolute;
    top: 0;
    left: 0;
    opacity: 0;
    white-space: pre;
    display: inline-block;
  }
}
</style>
