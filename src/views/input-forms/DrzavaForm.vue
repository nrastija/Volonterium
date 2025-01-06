<template>
    <h1 class="start-title"> Forma za unošenje države u bazu podataka</h1>

    <form class = "input-form">
        <div class="form-group">
            <label for="Naziv">Naziv:</label>
            <input type="text" class="form-control" id="naziv" v-model="naziv">
        </div>

        <button type="submit" @click="saveDrzava">Spremi državu u BP</button>
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
        saveDrzava() {
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

<style scoped>
.input-form{
    margin: 0 auto;
    width: 30%;
    padding: 20px;
    border: 1px solid #ccc;
    border-radius: 10px;
    margin-top: 20px;

    background-color: rgb(25, 81, 88);
    border-radius: 30px;

    display: flex;
    flex-direction: column;
    text-align: left;
    align-items: center;
    justify-content: center;
}


h1{
    margin-top: 3rem;
}

.form-group{
    margin: 10px 0;
    
    input{
        margin-left: 30px;
    }
}
</style>