<template>
  <button
    class="analog-toggle"
    type="button"
    role="switch"
    :aria-checked="modelValue"
    :class="{ on: modelValue }"
    @click="toggle"
  >
    <span class="toggle-track">
      <span class="toggle-handle"></span>
    </span>
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
.analog-toggle {
  display: grid;
  gap: 0.35rem;
  justify-items: center;
  background: transparent;
  border: none;
  padding: 0;
  color: inherit;
  cursor: pointer;
}

.toggle-track {
  position: relative;
  width: 54px;
  height: 26px;
  border-radius: 999px;
  background: linear-gradient(180deg, #1d1a17, #2a2622);
  border: 1px solid #11100f;
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.6), 0 4px 8px rgba(0, 0, 0, 0.5);
}

.toggle-handle {
  position: absolute;
  width: 22px;
  height: 22px;
  border-radius: 50%;
  background: linear-gradient(160deg, #d5cbbd, #a89f94);
  top: 2px;
  left: 4px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
  transition: transform 0.15s ease-out, box-shadow 0.15s ease-out;
}

.analog-toggle.on .toggle-handle {
  transform: translateX(26px);
  box-shadow: 0 0 8px rgba(224, 194, 92, 0.6), 0 2px 6px rgba(0, 0, 0, 0.4);
}

.analog-toggle.on .toggle-track {
  background: linear-gradient(180deg, #2f2a1f, #453a24);
  border-color: #1a140c;
}
</style>
