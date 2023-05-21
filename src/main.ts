import { createApp } from "vue";
import App from "./App.vue";
import setupPlugins from "./plugins";
import router, { setupRouter } from "./router";
import './styles.css'
import './plugins/tailwindcss/tailwind.css'

async function projectStart() {
    const app = createApp(App)
    setupPlugins(app)
    setupRouter(app)
    await router.isReady()
    app.mount('#app')
}

await projectStart()