<template>
    <TableView 
      apiEndpoint="http://127.0.0.1:3000/api/organizator" 
      naslov="Tablica: Organizator" 
      :headers="['ID', 'Naziv', 'Kontakt Osoba', 'Telefon', 'Mail']" 
    />
  </template>
  
  <script>
  import TableView from "../TableView.vue"; 
  
  export default {
    components: {
      TableView,
    },
  };
  </script>
  