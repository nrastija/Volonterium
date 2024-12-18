<template>
    <div class="table-container">
      <h1>Tablica Korisnika</h1>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Ime</th>
            <th>Prezime</th>
            <th>Email</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(korisnik, index) in korisnici" :key="index">
            <td>{{ korisnik.id }}</td>
            <td>{{ korisnik.ime }}</td>
            <td>{{ korisnik.prezime }}</td>
            <td>{{ korisnik.email }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>
  
  <script>
  export default {
    name: "App",
    data() {
      return {
        korisnici: [
          { id: 1, ime: "Ivan", prezime: "Ivić", email: "ivan.ivic@example.com" },
          { id: 2, ime: "Ana", prezime: "Anić", email: "ana.anic@example.com" },
          { id: 3, ime: "Marko", prezime: "Markić", email: "marko.markic@example.com" },
          { id: 4, ime: "Petra", prezime: "Petrić", email: "petra.petric@example.com" },
        ],
      };
    },
  };
  </script>