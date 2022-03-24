import { createRouter, createWebHistory } from 'vue-router'

import Counter from '../views/Counter.vue'
import Contract from '../views/Contract.vue'

const routerHistory = createWebHistory()
const routes = [
  { path: '/', component: Counter },
  { path: '/counter', component: Counter },
  { path: '/contract', component: Contract }
]

const router = createRouter({
  history: routerHistory,
  routes
})

export default router
