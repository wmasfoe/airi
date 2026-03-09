import type { RouteRecordRaw } from 'vue-router'

import NProgress from 'nprogress'

import { createPinia } from 'pinia'
import { setupLayouts } from 'virtual:generated-layouts'
import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { routes } from 'vue-router/auto-routes'

import App from './App.vue'

import { i18n } from './modules/i18n'

import '@proj-airi/font-cjkfonts-allseto/index.css'
import '@proj-airi/font-xiaolai/index.css'
import '@unocss/reset/tailwind.css'
import 'vue-sonner/style.css'
import 'uno.css'

const pinia = createPinia()

// TODO: vite-plugin-vue-layouts is long deprecated, replace with another layout solution
const routeRecords = setupLayouts(routes as RouteRecordRaw[])
const router = createRouter({ routes: routeRecords, history: createWebHistory() })

router.beforeEach((to, from) => {
  if (to.path !== from.path)
    NProgress.start()
})

router.afterEach(() => {
  NProgress.done()
})

createApp(App)
  .use(router)
  .use(pinia)
  .use(i18n)
  .mount('#app')
