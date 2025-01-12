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
      this.volonteri = await (await fetch("/api/volonteri")).json();
      this.dogadaji = await (await fetch("/api/dogadaji")).json();
    },
    async saveVolonterDogadaj() {
      if (!this.id_volonter || !this.id_dogadaj || !this.broj_sati || !this.status) {
        alert("Molimo unesite sva polja!");
        return;
      }

    },
  },
};
</script>
