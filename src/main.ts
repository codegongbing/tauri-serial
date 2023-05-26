import { createApp } from "vue";
import App from "./App.vue";
import setupPlugins from "./plugins";
import router, { setupRouter } from "./router";
import './plugins/tailwindcss/tailwind.css'
import './styles.scss'

// declare global {
//     interface Window {
//         __TAURI__: any;
//     }
// }

async function projectStart() {
    // @ts-ignore
    if (window.__TAURI__) {
        document.body.style.cssText = 'zoom: 1.25;'
    }
    const app = createApp(App)
    setupPlugins(app)
    setupRouter(app)
    await router.isReady()
    app.mount('#app')
}

await projectStart()