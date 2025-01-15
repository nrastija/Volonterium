<template>
  <TableView 
    apiEndpoint="http://127.0.0.1:3000/api/grad" 
    naslov="Tablica: Grad" 
    :headers="['ID', 'Naziv', 'Država ID']" 
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
