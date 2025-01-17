<template>
    <h1 class="start-title">Forma za unos povratnih informacija</h1>

    <form class="input-form">
        <div class="form-group">
            <label for="id_volonter">Volonter:</label>
            <select class="form-control" id="id_volonter" v-model="id_volonter" required>
                <option v-for="volonter in volonteri" :key="volonter.id" :value="volonter.id">
                    {{ volonter.ime + " " + volonter.prezime }}
                </option>
            </select>
        </div>

        <div class="form-group">
            <label for="id_dogadaj">Događaj:</label>
            <select class="form-control" id="id_dogadaj" v-model="id_dogadaj" required>
                <option v-for="dogadaj in dogadaji" :key="dogadaj.id" :value="dogadaj.id">
                    {{ dogadaj.naziv }}
                </option>
            </select>
        </div>

        <div class="form-group">
          <label for="ocjena">Ocjena (1-5): <span style="margin-left: 20px; font-weight: bold;"> {{ ocjena }}</span></label>
          <input 
              type="range" 
              id="ocjena" 
              class="form-control slider" 
              v-model="ocjena" 
              min="1" 
              max="5" 
              step="1"
          />
        </div>


        <div class="form-group">
            <label for="komentar">Komentar:</label>
            <textarea class="form-control" id="komentar" v-model="komentar"></textarea>
        </div>

        <button type="submit" @click.prevent="savePovratnaInformacija">Spremi povratnu informaciju</button>
    </form>
</template>

<script>
export default {
  data() {
    return {
      id_volonter: null,
      id_dogadaj: null,
      ocjena: null,
      komentar: "",
      ocjena: 3,
      volonteri: [],
      dogadaji: [],
    };
  },
  created() {
    this.fetchPodaci();
  },
  methods: {
    async fetchPodaci() {
      try {
            const response_volonteri = await fetch("http://127.0.0.1:3000/api/volonter");
            const response_dogadaji = await fetch("http://127.0.0.1:3000/api/dogadaj");
            if (response_volonteri.ok && response_dogadaji.ok) {
                this.volonteri = await response_volonteri.json();
                this.dogadaji = await response_dogadaji.json();
            } else {
                console.error("Greška prilikom dohvaćanja jedne ili više tablice:", response_dogadaji.statusText, response_volonteri.statusText);
            }
        } catch (error) {
            console.error("Greška:", error);
        }
    },
    async savePovratnaInformacija() {
      if (!this.id_volonter || !this.id_dogadaj || !this.ocjena) {
        alert("Molimo unesite obavezna polja!");
        return;
      }
    
      if (this.ocjena < 1 || this.ocjena > 5) {
        alert("Ocjena mora biti između 1 i 5!");
        return;
      }

      // Validacija opisa za sigurnost   
      if (!this.validirajUnos(this.id_volonter) || !this.validirajUnos(this.id_dogadaj) ||  !this.validirajUnos(this.ocjena)) {
            alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
            return;
        }

        const volonterDogadajData = {
            ocjena: this.ocjena,
            komentar: this.komentar,
            id_volonter: this.id_volonter,
            id_dogadaj: this.id_dogadaj,
        };

          try {
              const response = await fetch("http://127.0.0.1:3000/api/povratna-informacija", {
                  method: "POST",
                  headers: {
                      "Content-Type": "application/json",
                  },
                  body: JSON.stringify(volonterDogadajData),
              });

              if (response.ok) {
                  alert(`Povratna informacija uspješno unesena u bazu! Pokrenut je trigger u bazi! Automatski je zapisan datum unosa povratne informacije!`);
                  this.resetForm();
              } else {
                  const errorMessage = await response.text();
                  alert(`Greška prilikom unosa zapisa: ${errorMessage}`);
              }
          } catch (error) {
              console.error("Greška prilikom unosa zapisa:", error);
              alert("Došlo je do greške prilikom unosa zapisa.");
          }
        }, 

        validirajUnos(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
        },

        resetForm() {
          this.ocjena = 3;
          this.komentar =  "";
          this.id_volonter = null;
          this.id_dogadaj = null;
        },
  },
};
</script>




