<template>
    <div class="table-wrapper">
      <h1 class="start-title">Prikaz svih tablica</h1>
  
      <div class="table-container" v-for="(table, index) in tables" :key="index">
        <h2>{{ table.title }}</h2>
        <table class="data-table">
          <thead>
            <tr>
              <th v-for="(header, index) in table.headers" :key="index">
                {{ header }}
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(row, rowIndex) in table.data" :key="rowIndex">
              <td v-for="(value, key) in row" :key="key">
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
  data() {
    return {
      tables: [], // Čuva informacije o svim tablicama
    };
  },
  created() {
    this.loadTables();
  },
  methods: {
    async loadTables() {
      try {
        // Pozivanje svih API endpointa za tablice
        const drzave = await (await fetch("/api/drzave")).json();
        const organizatori = await (await fetch("/api/organizatori")).json();
        const dogadaji = await (await fetch("/api/dogadaji")).json();
        const volonteri = await (await fetch("/api/volonteri")).json();
        const vjestine = await (await fetch("/api/vjestine")).json();

        // Dodavanje tablica u niz
        this.tables = [
          {
            title: "Države",
            headers: ["ID", "Naziv"], // Naslovi kolona
            data: drzave, // Podaci iz API-ja
          },
          {
            title: "Organizatori",
            headers: ["ID", "Naziv", "Kontakt Osoba", "Telefon", "Mail"],
            data: organizatori,
          },
          {
            title: "Događaji",
            headers: ["ID", "Naziv", "Datum i Vrijeme", "Opis", "Potrebni Volonteri"],
            data: dogadaji,
          },
          {
            title: "Volonteri",
            headers: ["ID", "Ime", "Prezime", "Mail", "Telefon", "Datum Pridruživanja"],
            data: volonteri,
          },
          {
            title: "Vještine",
            headers: ["ID", "Naziv", "Opis"],
            data: vjestine,
          },
        ];
      } catch (error) {
        console.error("Greška prilikom dohvaćanja podataka:", error);
      }
    },
  },
};
</script>

<style scoped>
.table-wrapper {
  padding: 2rem;
  font-family: Arial, sans-serif;
}

.start-title {
  text-align: center;
  margin-bottom: 2rem;
  color: #ffffff;
}

.table-container {
  margin-bottom: 2rem;
  background-color: #1e1e2e;
  padding: 1rem;
  border-radius: 10px;
  box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.2);
}

h2 {
  color: #ffffff;
  text-align: center;
  margin-bottom: 1rem;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
}

.data-table th,
.data-table td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}

.data-table th {
  background-color: #535bf2;
  color: white;
}

.data-table tr:nth-child(even) {
  background-color: #f2f2f2;
}

.data-table tr:hover {
  background-color: #ddd;
}

.data-table td {
  color: #333;
}
</style>
