import { App } from "vue";


const setupPinia = (app: App) => {
    const pinia = createPinia()
    app.use(pinia)
}

export default setupPinia