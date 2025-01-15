<template>
    <h1 class="start-title"> Forma za unošenje volontera u bazu podataka</h1>

    <form class = "input-form">
        <div class="form-group">
            <label for="Ime">Ime:</label>
            <input type="text" class="form-control" id="ime" v-model="ime" required>
        </div>
        <div class="form-group">
            <label for="Prezime">Prezime:</label>
            <input type="text" class="form-control" id="prezime" v-model="prezime" required>
        </div>
        <div class="form-group">
            <label for="Mail">Mail:</label>
            <input type="text" class="form-control" id="mail" v-model="mail" required>
        </div>
        <div class="form-group">
            <label for="Telefon">Telefon:</label>
            <input type="text" class="form-control" id="telefon" v-model="telefon">
        </div>

        <button type="submit" @click.prevent="SaveVolonter">Spremi volontera u BP</button>
    </form>
</template>

<script>
export default {
    naziv: "Unos Vještine",
    opis: "Unos opisa",
    data() {
        return {
        ime: "",
        prezime: "",
        mail: "",
        telefon: "",
        datum_pridruzivanja: "",
        };
    },
    methods: {
        SaveVolonter() {
            if (!this.ime || !this.prezime || !this.mail) {
                alert("Molimo popunite sva obavezna polja!");
                return;
            }

            // Validacija opisa za sigurnost   
            if (!this.validirajUnos(this.ime) || !this.validirajUnos(this.prezime) || !this.validirajUnos(this.mail) || !this.validirajUnos(this.telefon)) { 
                alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
                return;
            }

            const datum_pridruzivanja = new Date().getTime();
            
            /*
            Logika za unos baze
            */

            alert("Volonter "+ this.ime + " " + this.prezime + " je uspješno unesen u bazu podataka!");
        },

        validirajUnos(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
        },
    },
};
</script>