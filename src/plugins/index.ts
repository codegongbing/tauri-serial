import { App } from "vue"
import setupElementPlus from "./elementplus"
import setupTailwindcss from "./tailwindcss"


const setupPlugins = (app: App) => {
    setupTailwindcss()
    setupElementPlus(app)
}

export default setupPlugins