import { RouteRecordRaw } from "vue-router";

const routes = [
    {
        path: '/',
        name: 'App',
        component: () => import('@/App.vue')
    }
] as RouteRecordRaw[]

export default routes