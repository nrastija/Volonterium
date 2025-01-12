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
import DogadajOrganizator from '../views/input-forms/DogadajOrganizator.vue';
import VolonterVjestina from '../views/input-forms/VolonterVjestina.vue';
import Error404 from '../views/errors/Error404.vue';
import VolonterDogadaj from '../views/input-forms/VolonterDogadaj.vue';
import PovratnaInformacija from '../views/input-forms/PovratnaInformacija.vue';

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
  
  //Ruta za slozene forme
  { path: '/forms/dogadaj-organizator', name: 'DogadajOrganizatorForm', component: DogadajOrganizator, meta: { title: "Forma za unos organizatora događaja" } },
  { path: '/forms/volonter-vjestina', name: 'VolonterVjestinaForm', component: VolonterVjestina, meta: { title: "Forma za unos vještine volontera" } },
  { path: '/forms/volonter-dogadaj', name: 'VolonterDogadajForm', component: VolonterDogadaj, meta: { title: "Forma za povezivanje volontera s događajem na kojem sudjeluje" } },
  { path: '/forms/povratna-informacija', name: 'PovratnaInformacijaForm', component: PovratnaInformacija, meta: { title: "Forma za unos povratne informacije" } },

  //Hvatanje 404
  { path: '/:pathMatch(.*)*', name: 'NotFound', component: Error404, meta: { title: "404 - Stranica nije pronađena" } },

];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
