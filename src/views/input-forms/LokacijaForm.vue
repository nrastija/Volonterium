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

        <button type="submit" class="btn-submit" @click.prevent="SaveLokacija">Spremi lokaciju u BP</button>
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
      try {
            const response = await fetch("http://127.0.0.1:3000/api/grad");
            if (response.ok) {
                this.gradovi = await response.json();
            } else {
                console.error("Greška prilikom dohvaćanja država:", response.statusText);
            }
        } catch (error) {
            console.error("Greška:", error);
        }
    },

    async SaveLokacija() {
        if (!this.naziv || !this.id_grad) {
            alert("Molimo unesite i odaberite sva polja!");
            return;
        }

        // Validacija opisa za sigurnost   
        if (!this.validirajUnos(this.naziv)) {
            alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
            return;
        }

        const lokacijaData = {
            adresa: this.adresa,
            id_grad: this.id_grad,
        };

          try {
              const response = await fetch("http://127.0.0.1:3000/api/lokacija", {
                  method: "POST",
                  headers: {
                      "Content-Type": "application/json",
                  },
                  body: JSON.stringify(lokacijaData),
              });

              if (response.ok) {
                  alert(`Lokacija sa adresom "${this.adresa}" je uspješno unesena u bazu podataka!`);
                  this.resetForm();
              } else {
                  const errorMessage = await response.text();
                  alert(`Greška prilikom unosa lokacije: ${errorMessage}`);
              }
          } catch (error) {
              console.error("Greška prilikom unosa lokacije:", error);
              alert("Došlo je do greške prilikom unosa lokacije.");
          }
        },
        
      validirajUnos(opis) {
              // Regex za zabranjene znakove
              const zabranjeniZnakovi = /['"%;(){}<>]/;
              return !zabranjeniZnakovi.test(opis);
        },
  },
};
</script>