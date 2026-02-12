<template>
  <section class="analog-panel analog-demo">
    <header class="demo-header">
      <h3>Analog Controls</h3>
      <p class="analog-label">Prototype rack module UI</p>
    </header>

    <div class="demo-grid">
      <AnalogIndicator :state="powerOn" label="Power" color="green" />
      <AnalogToggle v-model="powerOn" label="Bypass" />
      <AnalogButton v-model="warmMode" label="Warm" />
    </div>

    <div class="demo-grid knobs">
      <AnalogKnob v-model="drive" label="Drive" :min="0" :max="10" :step="0.1" />
      <AnalogKnob v-model="tone" label="Tone" :min="0" :max="100" :step="1" units="%" />
      <AnalogKnob v-model="mix" label="Mix" :min="0" :max="1" :step="0.01" units="" />
      <AnalogKnob v-model="output" label="Output" :min="-12" :max="12" :step="0.5" units="dB" />
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import AnalogButton from './AnalogButton.vue'
import AnalogIndicator from './AnalogIndicator.vue'
import AnalogKnob from './AnalogKnob.vue'
import AnalogToggle from './AnalogToggle.vue'

const powerOn = ref(true)
const warmMode = ref(false)
const drive = ref(4.2)
const tone = ref(60)
const mix = ref(0.35)
const output = ref(2)
</script>

<style lang="scss" scoped>
.analog-demo {
  padding: 1.5rem;
  border-radius: 18px;
  display: grid;
  gap: 1.2rem;
}

.demo-header {
  display: flex;
  align-items: baseline;
  gap: 1rem;
  justify-content: space-between;
}

.demo-header h3 {
  margin: 0;
  font-size: 1.1rem;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.demo-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(80px, 1fr));
  gap: 1.2rem;
  align-items: center;
}

.demo-grid.knobs {
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
}
</style>
