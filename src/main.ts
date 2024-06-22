import './assets/output.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'
import '@fortawesome/fontawesome-free/js/all'
import PrimeVue from 'primevue/config';
import Aura from '@primevue/themes/aura';
import { InstallCodemirro } from "codemirror-editor-vue3" 
const app = createApp(App)
app.use(PrimeVue, {
    theme: {
        preset: Aura
    }
});
app.use(InstallCodemirro)
app.use(createPinia())
app.use(router)

app.mount('#app')
