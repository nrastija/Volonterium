import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import FormChooser from '../views/FormChooser.vue';
import DogadajForm from '../views/input-forms/DogadajForm.vue';
import GradForm from '../views/input-forms/GradForm.vue';
import LokacijaForm from '../views/input-forms/LokacijaForm.vue';
import OrganizatorForm from '../views/input-forms/OrganizatorForm.vue';
import DrzavaForm from '../views/input-forms/DrzavaForm.vue';
import VolonterForm from '../views/input-forms/VolonterForm.vue';
import VjestinaForm from '../views/input-forms/VjestinaForm.vue';
import Error404 from '../views/errors/Error404.vue';
import TableChooser from '../views/TableChooser.vue';
import GradTable from '../views/table-views/GradTable.vue';
import LokacijaTable from '../views/table-views/LokacijaTable.vue';
import OrganizatorTable from '../views/table-views/OrganizatorTable.vue';
import DrzavaTable from '../views/table-views/DrzavaTable.vue';
import DogadajTable from '../views/table-views/DogadajTable.vue';
import VolonterTable from '../views/table-views/VolonterTable.vue';
import VjestinaTable from '../views/table-views/VjestinaTable.vue';
import DogadajOrganizatorTable from '../views/table-views/DogadajOrganizatorTable.vue';
import VolonterVjestinaTable from '../views/table-views/VolonterVjestinaTable.vue';
import VolonterDogadajTable from '../views/table-views/VolonterDogadajTable.vue';
import PovratnaInformacijaForm from '../views/input-forms/PovratnaInformacijaForm.vue';
import DogadajOrganizatorForm from '../views/input-forms/DogadajOrganizatorForm.vue';
import VolonterVjestinaForm from '../views/input-forms/VolonterVjestinaForm.vue';
import VolonterDogadajForm from '../views/input-forms/VolonterDogadajForm.vue';
import PovratnaInformacijaTable from '../views/table-views/PovratnaInformacijaTable.vue';

const routes = [
  { path: '/', name: 'Home', component: Home, meta: { title: "Dobrodošli u Volonterium!" }},
  { path: '/forms', name: 'Forms', component: FormChooser, meta: { title: "Izbornik formi" } },
  { path: '/tables', name: 'Tables', component: TableChooser, meta: { title: "Izbornik tablica" } },

  //Rute za tablice
  { path: '/tables/drzava', name: 'DrzavaTable', component: DrzavaTable, meta: { title: "Tablica država" } },
  { path: '/tables/organizator', name: 'OrganizatorTable', component: OrganizatorTable, meta: { title: "Tablica organizatora" } },
  { path: '/tables/dogadaj', name: 'DogadajTable', component: DogadajTable, meta: { title: "Tablica događaja" } },
  { path: '/tables/volonter', name: 'VolonterTable', component: VolonterTable, meta: { title: "Tablica volontera" } },
  { path: '/tables/vjestina', name: 'VjestinaTable', component: VjestinaTable, meta: { title: "Tablica vještina" } },

  //Rute za slozene tablice
  { path: '/tables/grad', name: 'GradTable', component: GradTable, meta: { title: "Tablica gradova" } },
  { path: '/tables/lokacija', name: 'LokacijaTable', component: LokacijaTable, meta: { title: "Tablica lokacija" } },
  { path: '/tables/dogadaj-organizator', name: 'DogadajOrganizatorTable', component: DogadajOrganizatorTable, meta: { title: "Tablica organizatora događaja" } },
  { path: '/tables/volonter-vjestina', name: 'VolonterVjestinaTable', component: VolonterVjestinaTable, meta: { title: "Tablica vještina volontera" } },
  { path: '/tables/dogadaj-volonter', name: 'DogadajVolonterTable', component: VolonterDogadajTable, meta: { title: "Tablica volontera na događajima" } },
  { path: '/tables/povratna-informacija', name: 'PovratnaInformacijaTable', component: PovratnaInformacijaTable, meta: { title: "Tablica povratnih informacija" } },

  //Rute za forme
  { path: '/forms/drzava', name: 'DrzavaForm', component: DrzavaForm, meta: { title: "Forma za unos države" } },
  { path: '/forms/organizator', name: 'OrganizatorForm', component: OrganizatorForm, meta: { title: "Forma za unos organizatora" } },
  { path: '/forms/dogadaj', name: 'DogadajForm', component: DogadajForm, meta: { title: "Forma za unos događaja" } },
  { path: '/forms/volonter', name: 'VolonterForm', component: VolonterForm, meta: { title: "Forma za unos volontera" } },
  { path: '/forms/vjestina', name: 'VjestinaForm', component: VjestinaForm, meta: { title: "Forma za unos vještine" } },
  
  //Ruta za slozene forme
  { path: '/forms/lokacija', name: 'LokacijaForm', component: LokacijaForm, meta: { title: "Forma za unos lokacije" } },
  { path: '/forms/grad', name: 'GradForm', component: GradForm, meta: { title: "Forma za unos grada" } },
  { path: '/forms/dogadaj-organizator', name: 'DogadajOrganizatorForm', component: DogadajOrganizatorForm, meta: { title: "Forma za unos organizatora događaja" } },
  { path: '/forms/volonter-vjestina', name: 'VolonterVjestinaForm', component: VolonterVjestinaForm, meta: { title: "Forma za unos vještine volontera" } },
  { path: '/forms/volonter-dogadaj', name: 'VolonterDogadajForm', component: VolonterDogadajForm, meta: { title: "Forma za povezivanje volontera s događajem na kojem sudjeluje" } },
  { path: '/forms/povratna-informacija', name: 'PovratnaInformacijaForm', component: PovratnaInformacijaForm, meta: { title: "Forma za unos povratne informacije" } },

  //Hvatanje 404
  { path: '/:pathMatch(.*)*', name: 'NotFound', component: Error404, meta: { title: "404 - Stranica nije pronađena" } },

];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
