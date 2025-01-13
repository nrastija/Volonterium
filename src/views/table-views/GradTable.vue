<template>
  <TableView 
    apiEndpoint="/api/grad" 
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
