<template>
    <h1 class="start-title">Forma za unos podataka o volonteru i događaju</h1>

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
            <label for="broj_sati">Broj sati:</label>
            <input type="number" class="form-control" id="broj_sati" v-model="broj_sati" required />
        </div>

        <div class="form-group">
            <label for="status">Status:</label>
            <input type="text" class="form-control" id="status" v-model="status" required />
        </div>

        <button type="submit" @click.prevent="saveVolonterDogadaj">Spremi podatke u BP</button>
    </form>
</template>

<script>
export default {
  data() {
    return {
      id_volonter: null,
      id_dogadaj: null,
      broj_sati: null,
      status: "",
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
    async saveVolonterDogadaj() {
      if (!this.id_volonter || !this.id_dogadaj || !this.broj_sati || !this.status) {
        alert("Molimo unesite sva polja!");
        return;
      }


      // Validacija opisa za sigurnost   
      if (!this.validirajUnos(this.id_volonter) || !this.validirajUnos(this.id_dogadaj) || 
            !this.validirajUnos(this.broj_sati) || !this.validirajUnos(this.status)) {
            alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
            return;
        }

        const volonterDogadajData = {
            broj_sati: this.broj_sati,
            status: this.status,
            id_volonter: this.id_volonter,
            id_dogadaj: this.id_dogadaj,
        };

          try {
              const response = await fetch("http://127.0.0.1:3000/api/volonter-dogadaj", {
                  method: "POST",
                  headers: {
                      "Content-Type": "application/json",
                  },
                  body: JSON.stringify(volonterDogadajData),
              });

              if (response.ok) {
                  alert(`Informacija za volontera na događaju je uspješno dodana u bazu!`);
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
    
        validirajOpis(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
        },

        resetForm() {
          this.broj_sati = "";
          this.status =  "";
          this.id_volonter = null;
          this.id_dogadaj = null;
        },
  },
};
</script>
