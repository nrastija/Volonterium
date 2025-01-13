<template>
    <TableView 
      apiEndpoint="/api/volonter-dogadaj" 
      naslov="Tablica: Volonter_dogadaj" 
      :headers="['Broj sati', 'Status', 'Volonter ID', 'Dogadaj ID']" 
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
  