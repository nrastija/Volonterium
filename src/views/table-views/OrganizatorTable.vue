<template>
    <TableView 
      apiEndpoint="/api/organizator" 
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
  