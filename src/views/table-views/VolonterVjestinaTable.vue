<template>
    <TableView 
      apiEndpoint="http://127.0.0.1:3000/api/volonter-vjestina" 
      naslov="Tablica: Volonter_vjestina" 
      :headers="['Razina vještine', 'Volonter ID', 'Vjestina ID']" 
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
  