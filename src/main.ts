import { createApp } from 'vue'
import { addCollection } from '@iconify/vue'
import { icons as lucideIcons } from '@iconify-json/lucide'
import './assets/index.css'
import './assets/fonts.css'
import ui from '@nuxt/ui/vue-plugin'
import { createPinia } from 'pinia'
import App from './App.vue'
import { startupMark } from './utils/startup-trace'

startupMark('main.ts:start')

// Bundle the Lucide icon set locally so packaged Tauri builds do not need
// runtime network fetches for `i-lucide-*` icons used by Nuxt UI buttons.
addCollection(lucideIcons)

const app = createApp(App)
const pinia = createPinia()

app.use(ui)
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
