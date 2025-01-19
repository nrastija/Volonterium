<template>
    <TableView 
      apiEndpoint="http://127.0.0.1:3000/api/volonter-dogadaj" 
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
  