<template>
    <h1 class="start-title"> Forma za unošenje volonterskih događaja u bazu podataka</h1>

    <form class = "input-form">
        <div class="form-group">
            <label for="Naziv">Naziv:</label>
            <input type="text" class="form-control" id="naziv" v-model="naziv" required>
        </div>
        <div class="form-group">
            <label for="Datum_vrijeme">Datum i vrijeme:</label>
            <input type="datetime" class="form-control" id="datum_vrijeme" v-model="datum_vrijeme" required>
        </div>
        <div class="form-group">
            <label for="Opis">Opis (opcionalno):</label>
            <input type="text" class="form-control" id="opis" v-model="opis" >
        </div>
        <div class="form-group">
            <label for="Potrebni_volonteri">Broj potrebnih volontera</label>
            <input type="text" class="form-control" id="potrebni_volonteri" v-model="potrebni_volonteri" required>
        </div>

        <button type="submit" @click="SaveDogadaj">Spremi organizatora u BP</button>
    </form>
</template>

<script>
export default {
    naziv: "Unos Događaja",
    datum_vrijeme: "Unos datuma i vremena",
    opis: "Unos opisa",
    potrebni_volonteri: "Unos broja potrebnih volontera",
    methods: {
        SaveDogadaj() {
            if (!this.naziv || !this.datum_vrijeme || !this.potrebni_volonteri) {
                alert("Molimo popunite sva obavezna polja!");
                return;
            }

            // Validacija opisa za sigurnost   
            if (!this.validirajUnos(this.naziv) || !this.validirajUnos(this.datum_vrijeme) || !this.validirajUnos(this.opis) || !this.validirajUnos(this.potrebni_volonteri)) { 
                alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
                return;
            }
            
            /*
            Logika za unos baze
            */

            alert("Događaj " + this.naziv + " koji se odvija " + this.datum_vrijeme + " uspješno upisan u bazu!");
        },

        validirajUnos(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
        },
    },
};
</script>