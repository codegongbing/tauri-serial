import vue from '@vitejs/plugin-vue';
import AutoImportConfig from './auto-import';

const setupVitePlugins = () => {
    return [
        vue(),
        AutoImportConfig(),
    ]
}

export default setupVitePlugins