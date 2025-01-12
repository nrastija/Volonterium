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

        <button type="submit" class="btn-submit" @click="saveDogadajOrganizator">Spremi organizatora u BP</button>
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
      this.dogadaji = await (await fetch("api/dogadaji")).json();
      this.organizatori = await (await fetch("api/organizatori")).json();
      this.lokacije = await (await fetch("api/lokacije")).json();
    },
    async saveDogadajOrganizator() {
        if (!this.uloga_organizatora || !this.id_dogadaj || !this.id_organizator || !this.id_lokacija) {
            alert("Molimo unesite sva polja!");
            return;
        }

        // Validacija opisa za sigurnost   
        if (!this.validirajUnos(this.uloga_organizatora) || !this.validirajUnos(this.id_dogadaj) || 
            !this.validirajUnos(this.id_organizator) || !this.validirajUnos(this.id_lokacija)) {
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