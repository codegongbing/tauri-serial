import { App } from "vue"
import setupElementPlus from "./elementplus"
import setupTailwindcss from "./tailwindcss"
import setupPinia from "./pinia"


const setupPlugins = (app: App) => {
    setupTailwindcss()
    setupElementPlus(app)
    setupPinia(app)
}

export default setupPlugins