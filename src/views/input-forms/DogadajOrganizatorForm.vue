<template>
    <h1 class="start-title">Forma za unos organizatora događaja</h1>

    <form class="input-form">
        <div class="form-group">
            <label for="uloga_organizatora">Uloga organizatora:</label>
            <select class="form-control" id="uloga_organizatora" v-model="uloga_organizatora" required>
                <option v-for="uloga in uloge" :key="uloga" :value="uloga">
                    {{ uloga }}
                </option>
            </select>
        </div>
        <div class="form-group">
            <label for="adresa">Komentar:</label>
            <input type="text" v-model="komentar" required />
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
            <label for="id_organizator">Organizator:</label>
            <select class="form-control" id="id_organizator" v-model="id_organizator" required>
                <option v-for="organizator in organizatori" :key="organizator.id" :value="organizator.id">
                    {{ organizator.naziv }}
                </option>
            </select>
        </div>
        <div class="form-group">
            <label for="id_lokacija">Lokacija:</label>
            <select class="form-control" id="id_lokacija" v-model="id_lokacija" required>
                <option v-for="lokacija in lokacije" :key="lokacija.id" :value="lokacija.id">
                    {{ lokacija.adresa }}
                </option>
            </select>
        </div>

        <button type="submit" class="btn-submit" @click.prevent="SaveDogadajOrganizator">Spremi organizatora dogadaja u BP</button>
    </form>
</template>


<script>
export default {
  data() {
    return {
      uloge: [
        "Voditelj događaja",
        "Koordinator aktivnosti",
        "Tehnička podrška",
        "Marketinški menadžer",
        "Finansijski menadžer",
        "Logistički menadžer",
        "Sigurnosni menadžer",
        "Asistent organizatora",
        "Moderator",
        "Volonterski menadžer",
      ],
      komenmtar: "",
      uloga_organizatora: null,
      id_dogadaj: null,
      id_organizator: null,
      id_lokacija: null,
      dogadaji: [],
      organizatori: [],
      lokacije: [],
    };
  },
  created() {
    this.fetchPodaci();
  },
  methods: {
    async fetchPodaci() {
        try {
            const response_dogadaji = await fetch("http://127.0.0.1:3000/api/dogadaj");
            const response_organizatori = await fetch("http://127.0.0.1:3000/api/organizator");
            const response_lokacije = await fetch("http://127.0.0.1:3000/api/lokacija");
            if (response_dogadaji.ok && response_organizatori.ok && response_lokacije.ok) {
                this.dogadaji = await response_dogadaji.json();
                this.organizatori = await response_organizatori.json();
                this.lokacije = await response_lokacije.json();
            } else {
                console.error("Greška prilikom dohvaćanja jedne ili više tablice:", response_dogadaji.statusText, response_lokacije.statusText, response_organizatori.statusText);
            }
        } catch (error) {
            console.error("Greška:", error);
        }
    },
    async SaveDogadajOrganizator() {
        if (!this.uloga_organizatora || !this.komentar || !this.id_dogadaj || !this.id_organizator || !this.id_lokacija) {
            alert("Molimo unesite sva polja!");
            return;
        }

        // Validacija opisa za sigurnost   
        if (!this.validirajUnos(this.uloga_organizatora) || !this.validirajOpis(this.komentar) || !this.validirajUnos(this.id_dogadaj) || 
            !this.validirajUnos(this.id_organizator) || !this.validirajUnos(this.id_lokacija)) {
            alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
            return;
        }

        const dogadajOrganizatorData = {
            uloga_organizatora: this.uloga_organizatora,
            komentar: this.komentar,
            id_dogadaj: this.id_dogadaj,
            id_organizator: this.id_organizator,
            id_lokacija: this.id_lokacija,
        };

          try {
              const response = await fetch("http://127.0.0.1:3000/api/dogadaj-organizator", {
                  method: "POST",
                  headers: {
                      "Content-Type": "application/json",
                  },
                  body: JSON.stringify(dogadajOrganizatorData),
              });

              if (response.ok) {
                  alert(`Organizator sa ulogom "${this.uloga}" za događaj je uspješno unesen u bazu podataka!`);
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
        this.uloga_organizatora = "";
        this.komentar = "";
        this.id_dogadaj = null;
        this.id_organizator = null;
        this.id_lokacija = null;
      },
  },
};
</script>