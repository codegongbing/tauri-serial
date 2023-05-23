import { createApp } from "vue";
import App from "./App.vue";
import setupPlugins from "./plugins";
import router, { setupRouter } from "./router";
import './plugins/tailwindcss/tailwind.css'
import './styles.scss'

async function projectStart() {
    // 如果系统为mac，设置缩放125%
    // if (/macintosh|mac os x/i.test(navigator.userAgent)) {
    //     document.body.style.cssText = 'zoom: 1.25;'
    // }
    const app = createApp(App)
    setupPlugins(app)
    setupRouter(app)
    await router.isReady()
    app.mount('#app')
}

await projectStart()