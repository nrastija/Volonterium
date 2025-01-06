<template>
    <div class="table-container">
      <h1>Tablica Korisnika</h1>
      <div class="highlightMain">
      <table>
        <thead>
          <tr>
            <th v-for="(key, index) in columnKeys" :key="index">
              {{ key }}
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(korisnik, rowIndex) in korisniciData" :key="rowIndex">
            <td v-for="(value, colIndex) in korisnik" :key="colIndex">
              {{ value }}
            </td>
          </tr>
        </tbody>
      </table>
      </div>
    </div>
  </template>
  
  <script>
export default {
  name: "DynamicTable",
  data() {
    return {
      korisniciData: [
        { id: 1, ime: "Ivan", prezime: "Ivić", email: "ivan.ivic@example.com" },
        { id: 2, ime: "Ana", prezime: "Anić", email: "ana.anic@example.com" },
        { id: 3, ime: "Marko", prezime: "Markić", email: "marko.markic@example.com" },
      ],
    };
  },
  computed: {
    columnKeys() {
      return this.korisniciData.length > 0 ? Object.keys(this.korisniciData[0]) : [];
    },
  },
};
</script>

<style scoped>
.table-container {
  margin: 2rem auto;
  margin-top: 3rem;
  max-width: 600px;
  font-family: Arial, sans-serif;
}

table {
  width: 100%;
  border-collapse: collapse;
}

.highlightMain{
  background-color: rgb(25, 81, 88);
  padding: 30px;
  margin: auto;
  border-radius: 30px;
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