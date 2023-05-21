import { createRouter, createWebHistory } from "vue-router";
import routes from "./routes";
import { App } from "vue";


const router = createRouter({
    history: createWebHistory(),
    routes: [...routes]
})

export function setupRouter(app: App) {
    app.use(router)
}

export default router