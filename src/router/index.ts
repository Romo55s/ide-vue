import { createRouter, createWebHistory } from 'vue-router'
import Editor from '../components/Editor.vue'
import Welcome from '../views/Welcome.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Welcome
    },
    {
      path: '/editor',
      name: 'editor',
      component: Editor
    }
  ]
})

export default router
