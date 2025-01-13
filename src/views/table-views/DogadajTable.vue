<template>
    <TableView 
      apiEndpoint="/api/dogadaj" 
      naslov="Tablica: Dogadaj" 
      :headers="['ID', 'Naziv', 'Datum i vrijeme', 'Opis', 'Potrebni volonteri']" 
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
  