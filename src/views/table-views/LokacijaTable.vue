<template>
    <TableView 
      apiEndpoint="http://127.0.0.1:3000/api/lokacija" 
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
  