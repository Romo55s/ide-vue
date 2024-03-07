import { createRouter, createWebHistory } from 'vue-router'
import Editor from '../components/Editor.vue'
import Welcome from '../views/Welcome.vue'
import Errors from '../views/Errors.vue'
import Lexic from '../views/Lexico.vue'
import semantic from '../views/Semantico.vue'
import syntactic from '../views/Sintactico.vue'
import run from '../views/Run.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Welcome
    },
    {
      path: '/errors',
      name: 'errors',
      component: Errors
    },
    {
      path: '/editor',
      name: 'editor',
      component: Editor
    },
    {
      path: '/lexic',
      name: 'lexic',
      component: Lexic
    },
    {
      path: '/semantic',
      name: 'semantic',
      component: semantic
    },
    {
      path: '/syntactic',
      name: 'syntactic',
      component: syntactic
    },
    {
      path: '/run',
      name: 'run',
      component: run
    },
  ]
})

export default router
