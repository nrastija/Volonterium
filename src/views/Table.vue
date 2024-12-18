<template>
    <div class="table-container">
      <h1>Tablica Korisnika</h1>
      <table>
        <thead>
          <tr>
            <th v-for="(key, index) in columnKeys" :key="index">
              {{ key }}
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(korisnik, rowIndex) in korisnici" :key="rowIndex">
            <td v-for="(value, colIndex) in korisnik" :key="colIndex">
              {{ value }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>
  
  <script>
export default {
  name: "DynamicTable",
  data() {
    return {
      korisnici: [
        { id: 1, ime: "Ivan", prezime: "Ivić", email: "ivan.ivic@example.com" },
        { id: 2, ime: "Ana", prezime: "Anić", email: "ana.anic@example.com" },
        { id: 3, ime: "Marko", prezime: "Markić", email: "marko.markic@example.com" },
      ],
    };
  },
  computed: {
    columnKeys() {
      return this.korisnici.length > 0 ? Object.keys(this.korisnici[0]) : [];
    },
  },
};
</script>

<style>
.table-container {
  margin: 2rem auto;
  max-width: 600px;
  font-family: Arial, sans-serif;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: center;
}

td:hover {
  background-color: rgba(242, 242, 242, 0.5);
}

</style>