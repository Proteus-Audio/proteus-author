<template>
  <div class="relative inline-block">
    <input
      ref="textInput"
      type="text"
      class="mb-4 min-w-[10px] max-w-full border-none bg-transparent p-0 text-base outline-none"
      :style="`width: ${width}px;`"
      :placeholder="placeholder"
      :value="modelValue"
      @input="updateValue"
    />
    <div ref="sizer" class="invisible absolute top-0 left-0 inline-block whitespace-pre">
      {{ sizerText }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUpdated, ref } from 'vue'

interface Props {
  placeholder?: string
  modelValue: string
}

const emit = defineEmits(['update:modelValue'])

const props = defineProps<Props>()

const updateValue = (event: Event) => {
  emit('update:modelValue', (event.target as HTMLTextAreaElement).value)
}

const textInput = ref(null as HTMLElement | null)
const sizer = ref(null as HTMLElement | null)
const width = ref(0)

const sizerText = computed(() =>
  props.modelValue !== '' ? props.modelValue : props.placeholder || '',
)

const resize = () => {
  width.value = sizer.value?.offsetWidth || width.value
}

onMounted(() => {
  resize()
})

onUpdated(() => {
  resize()
})
</script>
