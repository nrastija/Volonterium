<template>
    <h1 class="start-title"> Forma za unošenje volonterskih događaja u bazu podataka</h1>

    <form class = "input-form">
        <div class="form-group">
            <label for="Naziv">Naziv:</label>
            <input type="text" class="form-control" id="naziv" v-model="naziv" required>
        </div>
        <div class="form-group">
            <label for="Datum_vrijeme">Datum i vrijeme (Format YYYY-MM-DDTHH:MM:SS [ISO 8601]):</label>
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

        <button type="submit" @click.prevent="SaveDogadaj">Spremi događaj u BP</button>
    </form>
</template>

<script>
export default {
    naziv: "Unos Događaja",
    datum_vrijeme: "Unos datuma i vremena",
    opis: "Unos opisa",
    potrebni_volonteri: "Unos broja potrebnih volontera",
    methods: {
    async SaveDogadaj() {
        if (!this.naziv || !this.datum_vrijeme || !this.potrebni_volonteri) {
            alert("Molimo popunite sva obavezna polja!");
            return;
        }

        try {
            // Send a POST request to the backend API
            const response = await fetch("http://127.0.0.1:3000/api/dogadaj", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    naziv: this.naziv,
                    datum_vrijeme: this.datum_vrijeme,
                    opis: this.opis || null, // Optional field
                    potrebni_volonteri: parseInt(this.potrebni_volonteri, 10),
                }),
            });

            if (response.ok) {
                alert(`Događaj "${this.naziv}" uspješno unesen u bazu podataka!`);
                // Clear the form fields
                this.naziv = "";
                this.datum_vrijeme = "";
                this.opis = "";
                this.potrebni_volonteri = "";
            } else {
                const errorMessage = await response.text();
                alert(`Greška prilikom unosa događaja: ${errorMessage}`);
            }
        } catch (error) {
            console.error("Error while saving event:", error);
            alert("Došlo je do greške prilikom unosa događaja.");
        }
    },
},

};
</script>