<template>
    <TableView 
      apiEndpoint="http://127.0.0.1:3000/api/povratna-informacija" 
      naslov="Tablica: Povratna_informacija" 
      :headers="['Ocjena', 'Komentar', 'Datum i vrijeme', 'Volonter ID', 'Dogadaj ID']" 
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
  