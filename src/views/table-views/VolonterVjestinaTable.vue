<template>
    <TableView 
      apiEndpoint="/api/volonter-vjestina" 
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
  