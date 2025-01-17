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

        <button type="submit" @click.prevent="SaveVjestina">Spremi vjestinu u BP</button>
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
      try {
            const response_volonteri = await fetch("http://127.0.0.1:3000/api/volonter");
            const response_vjestine = await fetch("http://127.0.0.1:3000/api/vjestina");
            if (response_volonteri.ok && response_vjestine.ok) {
                this.volonteri = await response_volonteri.json();
                this.vjestine = await response_vjestine.json();
            } else {
                console.error("Greška prilikom dohvaćanja jedne ili više tablice:", response_vjestine.statusText, response_volonteri.statusText);
            }
        } catch (error) {
            console.error("Greška:", error);
        }
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

        const volonterVjestinaData = {
            razina: this.razina_vjestine,
            id_volonter: this.id_volonter,
            id_vjestina: this.id_vjestina,
        };

          try {
              const response = await fetch("http://127.0.0.1:3000/api/volonter-vjestina", {
                  method: "POST",
                  headers: {
                      "Content-Type": "application/json",
                  },
                  body: JSON.stringify(volonterVjestinaData),
              });

              if (response.ok) {
                  alert(`Razina vještine za volontera je uspješno dodana u bazu!`);
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
        this.razina_vjestine = "";
        this.id_volonter = null;
        this.id_vjestina = null;
      },
  },
};
</script>