<template>
    <TableView 
      apiEndpoint="http://127.0.0.1:3000/api/dogadaj" 
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
  