<template>
    <h1 class="start-title"> Forma za unošenje organizatora volonterskih događaja u bazu podataka</h1>

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

        <button type="submit" @click.prevent="SaveOrganizator">Spremi organizatora u BP</button>
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
        async SaveOrganizator() {
            if (!this.naziv || !this.kontakt_osoba || !this.telefon || !this.mail) {
                alert("Molimo popunite sva obavezna polja!");
                return;
            }

            if (!this.validirajUnos(this.naziv) || !this.validirajUnos(this.kontakt_osoba) || !this.validirajUnos(this.telefon) || !this.validirajUnos(this.mail)) {
                alert("Unos sadrži nedozvoljene znakove. Pokušajte ponovo.");
                return;
            }

            try {
                const response = await fetch("http://127.0.0.1:3000/api/organizator", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        naziv: this.naziv,
                        kontakt_osoba: this.kontakt_osoba,
                        telefon: this.telefon,
                        mail: this.mail,
                    }),
                });

                if (response.ok) {
                    alert(`Organizator "${this.naziv}" uspješno je dodan u bazu podataka!`);
                    this.naziv = "";
                    this.kontakt_osoba = "";
                    this.telefon = "";
                    this.mail = "";
                } else {
                    const errorMessage = await response.text();
                    alert(`Greška prilikom unosa organizatora: ${errorMessage}`);
                }
            } catch (error) {
                console.error("Greška pri slanju podataka:", error);
                alert("Došlo je do greške prilikom unosa organizatora.");
            }
        },

        validirajUnos(input) {
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(input);
        },
    },
};
</script>