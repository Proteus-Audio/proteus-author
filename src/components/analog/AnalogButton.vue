<template>
  <button
    class="analog-button"
    type="button"
    :class="{ active: modelValue }"
    @click="toggle"
  >
    <span class="button-cap"></span>
    <span v-if="label" class="analog-label">{{ label }}</span>
  </button>
</template>

<script setup lang="ts">
interface Props {
  modelValue: boolean
  label?: string
}

const props = withDefaults(defineProps<Props>(), {
  label: '',
})

const emit = defineEmits(['update:modelValue'])

const toggle = () => {
  emit('update:modelValue', !props.modelValue)
}
</script>

<style lang="scss" scoped>
.analog-button {
  display: grid;
  gap: 0.35rem;
  justify-items: center;
  background: transparent;
  border: none;
  padding: 0;
  cursor: pointer;
  color: inherit;
}

.button-cap {
  width: 46px;
  height: 46px;
  border-radius: 10px;
  background:
    linear-gradient(145deg, #d6cdbf, #9f978b),
    linear-gradient(180deg, rgba(255, 255, 255, 0.1), rgba(0, 0, 0, 0.2));
  border: 1px solid #6d645a;
  box-shadow: inset 0 2px 4px rgba(255, 255, 255, 0.5), 0 6px 12px rgba(0, 0, 0, 0.4);
  position: relative;
  transition: transform 0.1s ease-out, box-shadow 0.1s ease-out;
}

.analog-button.active .button-cap {
  transform: translateY(2px);
  box-shadow: inset 0 2px 6px rgba(0, 0, 0, 0.6), 0 2px 4px rgba(0, 0, 0, 0.4);
  background: linear-gradient(145deg, #c9b56b, #8b6f26);
}
</style>
