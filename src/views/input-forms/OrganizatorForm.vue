<template>
    <h1 class="start-title"> Forma za unošenje orgnazatora volonterskih događaja u bazu podataka</h1>

    <form class = "input-form">
        <div class="form-group">
            <label for="Naziv">Naziv:</label>
            <input type="text" class="form-control" id="naziv" v-model="naziv" required>
        </div>
        <div class="form-group">
            <label for="Kontakt_osoba">Kontakt osoba:</label>
            <input type="text" class="form-control" id="kontakt_osoba" v-model="kontakt_osoba" required>
        </div>
        <div class="form-group">
            <label for="Telefon">Telefon kontakt osobe:</label>
            <input type="text" class="form-control" id="telefon" v-model="telefon" required>
        </div>
        <div class="form-group">
            <label for="Mail">Mail kontakt osobe:</label>
            <input type="text" class="form-control" id="mail" v-model="mail" required>
        </div>

        <button type="submit" @click="SaveOrganizator">Spremi organizatora u BP</button>
    </form>
</template>

<script>
export default {
    naziv: "Unos Organizatora",
    kontakt_osoba: "Unos kontakt osobe",
    telefon: "Unos telefona",
    mail: "Unos maila",
    data() {
        return {
        naziv: "",
        kontakt_osoba: "",
        telefon: "",
        mail: "",
        };
    },
    methods: {
        SaveOrganizator() {
            if (!this.naziv || !this.kontakt_osoba || !this.telefon || !this.mail) {
                alert("Molimo popunite sva obavezna polja!");
                return;
            }

            // Validacija opisa za sigurnost   
            if (!this.validirajUnos(this.naziv) || !this.validirajUnos(this.kontakt_osoba) || !this.validirajUnos(this.mail) || !this.validirajUnos(this.telefon)) { 
                alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
                return;
            }
            
            /*
            Logika za unos baze
            */

            alert("Organizator "+ this.naziv + " s kontakt osobom " + this.kontakt_osoba + " je uspješno unesen u bazu podataka!");
        },

        validirajUnos(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
        },
    },
};
</script>