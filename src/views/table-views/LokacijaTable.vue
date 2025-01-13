<template>
    <TableView 
      apiEndpoint="/api/lokacija" 
      naslov="Tablica: Lokacija" 
      :headers="['ID', 'Adresa', 'Grad ID']" 
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
  