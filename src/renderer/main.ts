import { createApp } from 'vue'
// import PrimeVue from 'primevue/config';
import { createPinia } from 'pinia';
import App from './App.vue'

import ElementPlus from 'element-plus'

const app = createApp(App);
const pinia = createPinia();

app.use(ElementPlus);
app.use(pinia);
app.mount('#app');
