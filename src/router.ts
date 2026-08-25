import { createRouter, createWebHashHistory } from 'vue-router'
import BookshelfView from './views/BookshelfView.vue'
import ReaderView from './views/ReaderView.vue'
import SettingsView from './views/SettingsView.vue'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', name: 'bookshelf', component: BookshelfView },
    { path: '/reader', name: 'reader', component: ReaderView },
    { path: '/settings', name: 'settings', component: SettingsView },
  ],
})

export default router
