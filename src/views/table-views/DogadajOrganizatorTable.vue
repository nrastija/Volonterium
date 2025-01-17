<template>
    <TableView 
      apiEndpoint="http://127.0.0.1:3000/api/dogadaj-organizator" 
      naslov="Tablica: Dogadaj_organizator" 
      :headers="['Uloga organizatora', 'ID dogadaj', 'ID organizator', 'ID lokacija']" 
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
  