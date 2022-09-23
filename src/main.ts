import { createApp } from "vue";
import "./style.css";
import "./assets/theme.css";
import "./assets/index.css";
import "./assets/fonts.css";
import { createPinia } from 'pinia'
import App from './App.vue'

import ElementPlus from 'element-plus'

const app = createApp(App)
const pinia = createPinia()

app.use(ElementPlus)
app.use(pinia)
app.mount('#app')
