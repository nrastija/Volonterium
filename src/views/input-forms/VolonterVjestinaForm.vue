<template>
    <h1 class="start-title"> Forma za unošenje vještina određenog volontera u bazu podataka</h1>

    <form class = "input-form">
        <div class="form-group">
            <label for="razina_vjestine">Razina vještine:</label>
                <select class="form-control" id="razina_vjestine" v-model="razina_vjestine" required>
                    <option v-for="razina in razine" :key="razina" :value="razina">
                        {{ razina }}
                    </option>
            </select>
        </div>

        <div class="form-group">
            <label for="id_volonter">Volonter:</label>
            <select class="form-control" id="id_volonter" v-model="id_volonter" required>
                <option v-for="volonter in volonteri" :key="volonter.id" :value="volonter.id">
                    {{ volonter.ime + " " + volonter.prezime }}
                </option>
            </select>
        </div>

        <div class="form-group">
            <label for="id_vjestina">Vještina:</label>
            <select class="form-control" id="id_vjestina" v-model="id_vjestina" required>
                <option v-for="vjestina in vjestine" :key="vjestina.id" :value="vjestina.id">
                    {{ vjestina.naziv }}
                </option>
            </select>
        </div>

        <button type="submit" @click="SaveVjestina">Spremi vjestinu u BP</button>
    </form>
</template>

<script>
export default {
  data() {
    return {
      razine: [
        "Početnik",
        "Srednje",
        "Napredno",
        "Ekspertno",
      ],
      razina_vjestine: null,
      id_volonter: null,
      id_vjestina: null,
      volonteri: [],
      vjestine: [],
    };
  },
  created() {
    this.fetchPodaci();
  },
  methods: {
    async fetchPodaci() {
      this.volonteri = await (await fetch("api/volonmteri")).json();
      this.vjestine = await (await fetch("api/vjestine")).json();
    },
    async saveDogadajOrganizator() {
        if (!this.razina_vjestine || !this.id_vjestina || !this.id_volonter) {
            alert("Molimo unesite sva polja!");
            return;
        }

        // Validacija opisa za sigurnost   
        if (!this.validirajUnos(this.razina_vjestine) || !this.validirajUnos(this.id_vjestina) ||  !this.validirajUnos(this.id_volonter)) {
            alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
            return;
        }

        /*
        Logika za unos baze
        */

        alert("Organizator događaja uspješno unesen u bazu!");
    },
    validirajOpis(opis) {
        // Regex za zabranjene znakove
        const zabranjeniZnakovi = /['"%;(){}<>]/;
        return !zabranjeniZnakovi.test(opis);
    },
  },
};
</script>