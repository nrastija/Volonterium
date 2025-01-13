<template>
    <TableView 
      apiEndpoint="/api/povratna-informacija" 
      naslov="Tablica: Povratna_informacija" 
      :headers="['Ocjena', 'Komentar', 'Datum', 'Volonter ID', 'Dogadaj ID']" 
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
  