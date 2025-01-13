<template>
    <TableView 
      apiEndpoint="/api/vjestina" 
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
  