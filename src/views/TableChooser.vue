<template>
  <div class="menu-wrapper">
    <h1 class="start-title">Pregled jednostavnih tablica u bazi:</h1>
    <div class="table-list">
      <div class="table-item" v-for="(tablica, index) in tablicePK" :key="index">
        <h2>{{ tablica.naziv }}</h2>
        <p>{{ tablica.opis }}</p>
        <router-link :to="tablica.route">
          <button class="view-button">Prikaži</button>
        </router-link>
      </div>
    </div>

    <h1 class="start-title">Pregled složenih tablica u bazi:</h1>
    <div class="table-list">
      <div class="table-item" v-for="(tablica, index) in tablicePKFK" :key="index">
        <h2>{{ tablica.naziv }}</h2>
        <p>{{ tablica.opis }}</p>
        <router-link :to="tablica.route">
          <button class="view-button">Prikaži</button>
        </router-link>
      </div>
    </div>
  </div>

</template>
<script>
export default {
  data() {
    return {
      tablicePK: [
        { naziv: "Države", opis: "Prikaz svih država.", route: "/tables/drzava" },
        { naziv: "Organizatori", opis: "Prikaz svih organizatora.", route: "/tables/organizator" },
        { naziv: "Događaji", opis: "Prikaz svih događaja.", route: "/tables/dogadaj" },
        { naziv: "Volonteri", opis: "Prikaz svih volontera.", route: "/tables/volonter" },
        { naziv: "Vještine", opis: "Prikaz svih vještina.", route: "/tables/vjestina" },
      ],
      tablicePKFK: [
        { naziv: "Gradovi", opis: "Prikaz svih gradova.", route: "/tables/grad" },
        { naziv: "Lokacije", opis: "Prikaz svih lokacija.", route: "/tables/lokacija"},
        { naziv: "Događaj organizator", opis: "Prikaz svih povezanih događaja i organizatora", route: "/tables/dogadaj-organizator"},
        { naziv: "Događaj volonter", opis: "Prikaz svih povezanih događaja i volontera", route: "/tables/dogadaj-volonter"},
        { naziv: "Volonter vještina", opis: "Prikaz svih povezanih volontera i vještina", route: "/tables/volonter-vjestina"},
        { naziv: "Povratna informacija", opis: "Prikaz svih povratnih informacija volontera na događaj", route: "/tables/povratna-informacija"},
      ],

   };
  },
};
</script>

<style scoped>
.menu-wrapper {
  padding: 2rem;
  text-align: center;
}

.start-title {
  font-size: 2rem;
  margin-bottom: 2rem;
}

.table-list {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 2rem;
}

.table-item {
  background-color: #1e1e2e;
  padding: 1.5rem;
  border-radius: 12px;
  width: 250px;
  text-align: center;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  color: #ffffff;
}

.view-button {
  background-color: #535bf2;
  color: white;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.view-button:hover {
  background-color: #4042b8;
}
</style>
