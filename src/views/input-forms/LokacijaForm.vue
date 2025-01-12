<template>
    <h1 class="start-title"> Forma za unošenje lokacije u bazu podataka</h1>

    <form class="input-form">
        <div class="form-group">
            <label for="adresa">Adresa:</label>
            <input type="text" v-model="adresa" required />
        </div>
        <div class="form-group">
            <label for="id_grad">Grad:</label>
            <select v-model="id_grad" required>
                <option v-for="grad in gradovi" :key="grad.id" :value="grad.id">
                {{ grad.naziv }}
                </option>
            </select>
        </div>

        <button type="submit" class="btn-submit" @click="saveGrad">Spremi lokaciju u BP</button>
    </form>
</template>

<script>
export default {
  data() {
    return {
      adresa: "",
      id_grad: null,
      gradovi: [], 
    };
  },
  created() {
    this.fetchGradovi();
  },
  methods: {
    async fetchGradovi() {
      const response = await fetch("api/gradovi"); 
      this.gradovi = await response.json();
    },

    async saveLokacija() {
        if (!this.naziv || !this.id_grad) {
            alert("Molimo unesite i odaberite sva polja!");
            return;
        }

        // Validacija opisa za sigurnost   
        if (!this.validirajUnos(this.naziv)) {
            alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
            return;
        }

        /*
        Logika za unos baze
        */

        alert("Lokacija " + this.naziv + " je uspješno unesena u bazu podataka!");
    },
    validirajUnos(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
        },
  },
};
</script>