import { createApp } from "vue";
import App from "./App.vue";
import setupPlugins from "./plugins";
import router, { setupRouter } from "./router";
import './plugins/tailwindcss/tailwind.css'
import './styles.scss'

if (/macintosh|mac os x/i.test(navigator.userAgent)) {
    // @ts-ignore
    if (window.__TAURI__) {
        document.body.style.cssText = 'zoom: 1.25;'
    }
}

const app = createApp(App)
setupPlugins(app)
setupRouter(app)
app.mount('#app')

// tauri不支持top-level await，暂时删除异步启动
// async function projectStart() {
//     // @ts-ignore
//     if (window.__TAURI__) {
//         document.body.style.cssText = 'zoom: 1.25;'
//     }
//     const app = createApp(App)
//     setupPlugins(app)
//     setupRouter(app)
//     await router.isReady()
//     app.mount('#app')
// }

// await projectStart()