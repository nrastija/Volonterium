<template>
    <h1 class="start-title"> Forma za unošenje države u bazu podataka</h1>

    <form class = "input-form">
        <div class="form-group">
            <label for="Naziv">Naziv:</label>
            <input type="text" class="form-control" id="naziv" v-model="naziv">
        </div>

        <button type="submit" @click="SaveDrzava">Spremi državu u BP</button>
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
        SaveDrzava() {
            if (this.naziv === "") {
                alert("Molimo unesite naziv države!");
                return;
            }

            /*
            Logika za unos baze
            */

            alert("Država  " + this.naziv + " je uspješno unesena u bazu podataka!");
        },
    },
};

</script>
