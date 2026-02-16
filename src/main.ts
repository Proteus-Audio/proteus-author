import { createApp } from 'vue'
import './assets/theme.css'
import './assets/index.css'
import './assets/analog.css'
import './assets/fonts.css'
import { createPinia } from 'pinia'
import App from './App.vue'
import { startupMark } from './utils/startup-trace'

import ElementPlus from 'element-plus'

startupMark('main.ts:start')

const app = createApp(App)
const pinia = createPinia()

app.use(ElementPlus)
app.use(pinia)
startupMark('main.ts:before-mount')
app.mount('#app')
startupMark('main.ts:after-mount')

// The first rAF after mount is our practical first-paint approximation.
// It is stable across environments where Paint Timing API can be unreliable.
requestAnimationFrame(() => {
  startupMark('main.ts:first-paint-approx')
  startupMark('main.ts:first-animation-frame')
})
