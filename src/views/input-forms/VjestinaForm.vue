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
        async SaveVjestina() {
            if (!this.naziv) {
                alert("Molimo unesite naziv vještine!");
                return;
            }

            // Validacija inputa
            if (!this.validirajUnos(this.opis) || !this.validirajUnos(this.naziv)) {
                alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
                return;
            }

            const vjestinaData = {
                naziv: this.naziv,
                opis: this.opis || null, 
            };

            try {
                const response = await fetch("http://127.0.0.1:3000/api/vjestina", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(vjestinaData),
                });

                if (response.ok) {
                    alert(`Vještina "${this.naziv}" je uspješno unesena u bazu podataka!`);
                    this.resetForm();
                } else {
                    const errorMessage = await response.text();
                    alert(`Greška prilikom unosa vještine: ${errorMessage}`);
                }
            } catch (error) {
                console.error("Greška prilikom unosa vještine:", error);
                alert("Došlo je do greške prilikom unosa vještine.");
            }
        },

        validirajUnos(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
        },

        resetForm() {
            this.naziv = "";
            this.opis = "";
        },
    },
};

</script>
