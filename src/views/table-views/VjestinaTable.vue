<template>
    <TableView 
      apiEndpoint="http://127.0.0.1:3000/api/vjestina" 
      naslov="Tablica: Vještina" 
      :headers="['ID', 'Naziv', 'Opis']" 
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
  