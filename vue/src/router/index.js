import { createRouter, createWebHistory } from 'vue-router'

import Counter from '../views/Counter.vue'
import Contract from '../views/Contract.vue'
import Query from '../views/Query.vue'
import Execute from '../views/Execute.vue'

const routerHistory = createWebHistory()
const routes = [
  { path: '/', component: Counter },
  { path: '/counter', component: Counter },
  { path: '/contract', component: Contract },
  { path: '/query', component: Query },
  { path: '/execute', component: Execute }
]

const router = createRouter({
  history: routerHistory,
  routes
})

export default router
