import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Table from '../views/Table.vue';
import FormChooser from '../views/FormChooser.vue';

const routes = [
  { path: '/', name: 'Home', component: Home, meta: { title: "Dobrodošli u Volonterium!" }},
  { path: '/table', name: 'Table', component: Table, meta: { title: "Tablica" } },
  { path: '/forms', name: 'Forms', component: FormChooser, meta: { title: "Izbornik formi" } },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
