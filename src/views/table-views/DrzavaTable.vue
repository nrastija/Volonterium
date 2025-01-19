<template>
    <TableView 
      apiEndpoint="http://127.0.0.1:3000/api/drzava" 
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
  