<template>
    <TableView 
      apiEndpoint="/api/drzava" 
      naslov="Tablica: Država" 
      :headers="['ID', 'Naziv']" 
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
  