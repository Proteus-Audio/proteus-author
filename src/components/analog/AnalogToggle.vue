<template>
  <button
    class="grid cursor-pointer justify-items-center gap-1.5 border-none bg-transparent p-0"
    type="button"
    role="switch"
    :aria-checked="modelValue"
    @click="toggle"
  >
    <span
      class="relative block h-[26px] w-[54px] rounded-full border border-[#11100f] bg-[linear-gradient(180deg,#1d1a17,#2a2622)] shadow-[inset_0_2px_4px_rgba(0,0,0,0.6),0_4px_8px_rgba(0,0,0,0.5)]"
      :class="modelValue ? 'border-[#1a140c] bg-[linear-gradient(180deg,#2f2a1f,#453a24)]' : ''"
    >
      <span
        class="absolute top-[2px] left-1 size-[22px] rounded-full bg-[linear-gradient(160deg,#d5cbbd,#a89f94)] shadow-[0_2px_6px_rgba(0,0,0,0.4)] transition-all duration-150"
        :class="{
          'translate-x-[26px] shadow-[0_0_8px_rgba(224,194,92,0.6),0_2px_6px_rgba(0,0,0,0.4)]':
            modelValue,
        }"
      ></span>
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
