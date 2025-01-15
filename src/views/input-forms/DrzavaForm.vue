<template>
    <h1 class="start-title"> Forma za unošenje države u bazu podataka</h1>

    <form class = "input-form">
        <div class="form-group">
            <label for="Naziv">Naziv:</label>
            <input type="text" class="form-control" id="naziv" v-model="naziv">
        </div>

        <button type="submit" @click.prevent="SaveDrzava">Spremi državu u BP</button>
    </form>
</template>

<script>
export default {
    naziv: "Unos Države",
    data() {
        return {
        naziv: "",
        };
    },
    methods: {
        async SaveDrzava() {
            if (!this.naziv) {
                alert("Molimo unesite naziv države!");
                return;
            }

            try {
                const response = await fetch("http://127.0.0.1:3000/api/drzava", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({ naziv: this.naziv }),
                });

                if (response.ok) {
                    alert(`Država "${this.naziv}" je uspješno unesena u bazu podataka!`);
                    this.naziv = "";
                } else {
                    const errorMessage = await response.text();
                    alert(`Greška prilikom unosa države: ${errorMessage}`);
                }
            } catch (error) {
                console.error("Error while saving state:", error);
                alert("Došlo je do greške prilikom unosa države.");
            }

        },
    },
};

</script>
