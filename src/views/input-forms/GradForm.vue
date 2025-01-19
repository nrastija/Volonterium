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

        <button type="submit" class="btn-submit" @click.prevent="saveGrad">Spremi grad u BP</button>
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
        try {
            const response = await fetch("http://127.0.0.1:3000/api/drzava");
            if (response.ok) {
                this.drzave = await response.json();
            } else {
                console.error("Greška prilikom dohvaćanja država:", response.statusText);
            }
        } catch (error) {
            console.error("Greška:", error);
        }
      },

      async saveGrad() {
        if (!this.naziv || !this.id_drzava) {
            alert("Molimo unesite i odaberite sva polja!");
            return;
        }

        if (!this.validirajUnos(this.naziv)) {
            alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
            return;
        }

        const gradData = {
            naziv: this.naziv,
            id_drzava: this.id_drzava,
        };

        try {
            const response = await fetch("http://127.0.0.1:3000/api/grad", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(gradData),
            });

            if (response.ok) {
                alert(`Grad "${this.naziv}" je uspješno unesen u bazu podataka!`);
                this.resetForm();
            } else {
                const errorMessage = await response.text();
                alert(`Greška prilikom unosa grada: ${errorMessage}`);
            }
        } catch (error) {
            console.error("Greška prilikom unosa grada:", error);
            alert("Došlo je do greške prilikom unosa grada.");
        }
      },

      validirajUnos(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
      },

      resetForm() {
        this.naziv = "";
        this.id_drzava = null;
      },
    },
  };
  </script>
  