<template>
    <h1 class="start-title"> Forma za unošenje vještine u bazu podataka</h1>

    <form class = "input-form">
        <div class="form-group">
            <label for="Naziv">Naziv vještine:</label>
            <input type="text" class="form-control" id="naziv" v-model="naziv" required>
        </div>
        <div class="form-group">
            <label for="Opis">Opis (opcionalno):</label>
            <input type="text" class="form-control" id="opis" v-model="opis">
        </div>

        <button type="submit" @click.prevent="SaveVjestina">Spremi vještinu u BP</button>
    </form>
</template>

<script>
export default {
    naziv: "Unos Vještine",
    opis: "Unos opisa",
    data() {
        return {
        naziv: "",
        opis: "",
        };
    },
    methods: {
        SaveVjestina() {
            if (!this.naziv) {
                alert("Molimo unesite naziv vještine!");
                return;
            }

            // Validacija opisa za sigurnost   
            if (!this.validirajUnos(this.opis) || !this.validirajUnos(this.naziv)) {
                alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
                return;
            }

            /*
            Logika za unos baze
            */

            alert("Vještina " + this.naziv + " je uspješno unesena u bazu podataka!");
        },

        validirajOpis(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
        },
    },
};

</script>
