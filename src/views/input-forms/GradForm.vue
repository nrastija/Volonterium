<template>
    <h1 class="start-title"> Forma za unošenje grada u bazu podataka</h1>

    <form class="input-form">
        <div class="form-group">
            <label for="naziv">Naziv grada:</label>
            <input type="text" class="form-control" id="naziv" v-model="naziv" required>
        </div>
        <div class="form-group">
            <label for="id_drzava">Država:</label>
            <select class="form-control" id="id_drzava" v-model="id_drzava" required>
                <option v-for="drzava in drzave" :key="drzava.id" :value="drzava.id">
                    {{ drzava.naziv }}
                </option>
            </select>
        </div>

        <button type="submit" class="btn-submit" @click="saveGrad">Spremi grad u BP</button>
    </form>
</template>

  <script>
  export default {
    data() {
      return {
        naziv: "",
        id_drzava: null,
        drzave: [], 
      };
    },
    created() {
      this.fetchDrzave();
    },
    methods: {

      async fetchDrzave() {
        const response = await fetch("api/drzave"); // Endpoint za dohvat država
        this.drzave = await response.json();
      },

      async saveGrad() {
        if (!this.naziv || !this.id_drzava) {
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

            alert("Vještina " + this.naziv + " je uspješno unesena u bazu podataka!");
      },

      validirajUnos(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
        },
    },
  };
  </script>
  