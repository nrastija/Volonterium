<template>
    <TableView 
      apiEndpoint="/api/volonter" 
      naslov="Tablica: Volonter" 
      :headers="['ID', 'Ime', 'Prezime', 'Mail', 'Telefon', 'Datum pridruživanja']" 
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
  