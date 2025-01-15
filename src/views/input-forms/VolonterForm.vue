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
        <div class="form-group">
            <label for="DatumPridruzivanja">Datum pridruživanja (YYYY-MM-DD):</label>
            <input type="date" class="form-control" id="datumPridruzivanja" v-model="datum_pridruzivanja" required>
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
        async SaveVolonter() {
            // Validacija obaveznih polja
            if (!this.ime || !this.prezime || !this.mail || !this.datum_pridruzivanja) {
                alert("Molimo popunite sva obavezna polja!");
                return;
            }

            // Validacija inputa
            if (
                !this.validirajUnos(this.ime) ||
                !this.validirajUnos(this.prezime) ||
                !this.validirajUnos(this.mail) ||
                !this.validirajUnos(this.telefon)  ||
                !this.validirajUnos(this.datum_pridruzivanja)
            ) {
                alert("Atribut sadrži nedozvoljene znakove! Molimo pokušajte ponovo.");
                return;
            }

            // Priprema podataka
            const volonterData = {
                ime: this.ime,
                prezime: this.prezime,
                mail: this.mail,
                telefon: this.telefon,
                datum_pridruzivanja: this.datum_pridruzivanja, // Format za backend
            };

            try {
                // Slanje POST zahtjeva na backend
                const response = await fetch("http://127.0.0.1:3000/api/volonter", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(volonterData),
                });

                if (response.ok) {
                    alert(`Volonter "${this.ime} ${this.prezime}" je uspješno unesen u bazu podataka!`);
                    this.resetForm(); 
                } else {
                    const errorMessage = await response.text();
                    alert(`Greška prilikom unosa volontera: ${errorMessage}`);
                }
            } catch (error) {
                console.error("Greška prilikom unosa volontera:", error);
                alert("Došlo je do greške prilikom unosa volontera.");
            }
        },

        validirajUnos(opis) {
            // Regex za zabranjene znakove
            const zabranjeniZnakovi = /['"%;(){}<>]/;
            return !zabranjeniZnakovi.test(opis);
        },

        resetForm() {
            this.ime = "";
            this.prezime = "";
            this.mail = "";
            this.telefon = "";
        },
    },
};
</script>