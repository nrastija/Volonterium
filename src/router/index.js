import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Table from '../views/Table.vue';
import FormChooser from '../views/FormChooser.vue';
import DogadajForm from '../views/input-forms/DogadajForm.vue';
import GradForm from '../views/input-forms/GradForm.vue';
import LokacijaForm from '../views/input-forms/LokacijaForm.vue';
import OrganizatorForm from '../views/input-forms/OrganizatorForm.vue';
import DrzavaForm from '../views/input-forms/DrzavaForm.vue';
import VolonterForm from '../views/input-forms/VolonterForm.vue';
import VjestinaForm from '../views/input-forms/VjestinaForm.vue';

const routes = [
  { path: '/', name: 'Home', component: Home, meta: { title: "Dobrodošli u Volonterium!" }},
  { path: '/table', name: 'Table', component: Table, meta: { title: "Tablica" } },
  { path: '/forms', name: 'Forms', component: FormChooser, meta: { title: "Izbornik formi" } },

  //Rute za forme
  { path: '/forms/drzava', name: 'DrzavaForm', component: DrzavaForm, meta: { title: "Forma za unos države" } },
  { path: '/forms/grad', name: 'GradForm', component: GradForm, meta: { title: "Forma za unos grada" } },
  { path: '/forms/lokacija', name: 'LokacijaForm', component: LokacijaForm, meta: { title: "Forma za unos lokacije" } },
  { path: '/forms/organizator', name: 'OrganizatorForm', component: OrganizatorForm, meta: { title: "Forma za unos organizatora" } },
  { path: '/forms/dogadaj', name: 'DogadajForm', component: DogadajForm, meta: { title: "Forma za unos događaja" } },
  { path: '/forms/volonter', name: 'VolonterForm', component: VolonterForm, meta: { title: "Forma za unos volontera" } },
  { path: '/forms/vjestina', name: 'VjestinaForm', component: VjestinaForm, meta: { title: "Forma za unos vještine" } },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
