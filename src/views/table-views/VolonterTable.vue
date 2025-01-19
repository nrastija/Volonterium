<template>
    <TableView 
      apiEndpoint="http://127.0.0.1:3000/api/volonter" 
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
  