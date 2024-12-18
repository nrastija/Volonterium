import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Table from '../views/Table.vue';

const routes = [
  { path: '/', name: 'Home', component: Home },
  { path: '/table', name: 'Table', component: Table },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
