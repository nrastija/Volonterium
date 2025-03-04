<template>
  <div class="table-wrapper">
    <h1 class="start-title">{{ naslov }}</h1>
    <table class="data-table">
      <thead>
        <tr>
          <th v-for="(header, index) in headers" :key="index">{{ header }}</th>
          <th>Akcije</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(row, rowIndex) in data" :key="rowIndex">
          <td v-for="(value, key) in row" :key="key">{{ value }}</td>
          <td>
            <button @click="editRow(row)">Ažuriraj</button>
            <button @click="deleteRow(row)">Obriši</button>
          </td>
        </tr>
      </tbody>
    </table>

    <!-- Modal za ažuriranje -->
    <div v-if="showModal" class="modal-overlay">
      <div class="modal-content">
        <h2>Ažuriraj zapis</h2>
        <form @submit.prevent="updateRow">
          <div v-for="(value, key) in selectedRow" :key="key" class="form-group">
            <label :for="key">{{ key }}</label>
            <input 
              v-model="selectedRow[key]" 
              :id="key"
              :readonly="key === 'id' || key === 'datum_pridruzivanja'" 
            />
          </div>
          <div class="modal-actions">
            <button type="submit">Spremi promjene</button>
            <button type="button" @click="closeModal">Zatvori</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    apiEndpoint: String,
    naslov: String,
    headers: Array,
  },
  data() {
    return {
      data: [],
      showModal: false,
      selectedRow: null,
    };
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData() {
      try {
        const response = await fetch(this.apiEndpoint);
        if (response.ok) {
          this.data = await response.json();
        } else {
          console.error("Greška prilikom dohvaćanja podataka.");
        }
      } catch (error) {
        console.error("Greška:", error);
      }
    },
    async deleteRow(row) {
      this.selectedRow = { ...row };
      if (!confirm("Jeste li sigurni da želite obrisati zapis?")) return;

      try {
        let url = this.urlChooser();

        const response = await fetch(url, {
          method: "DELETE",
        });

        if (response.ok) {
          alert("Zapis uspješno obrisan.");
          this.fetchData();
        } else {
          console.error("Greška prilikom brisanja zapisa.");
        }
      } catch (error) {
        console.error("Greška:", error);
      }
    },

    editRow(row) {
      this.selectedRow = { ...row }; // Kopiraj podatke retka
      this.showModal = true; // Prikaži modal
    },
    async updateRow() {
      try {
        
        Object.keys(this.selectedRow).forEach(key => {
          if (key.startsWith("id_") && key !== "id") {
            this.selectedRow[key] = parseInt(this.selectedRow[key]);
          }
        });

        if (this.selectedRow.broj_sati) {
          this.selectedRow.broj_sati = parseInt(this.selectedRow.broj_sati);
        }

        if (this.selectedRow.datum_vrijeme) {
          if (!this.selectedRow.datum_vrijeme.includes("T")) {
            this.selectedRow.datum_vrijeme = this.selectedRow.datum_vrijeme.replace(" ", "T");
          }
        }

        var url = this.urlChooser();


        const response = await fetch(url, {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(this.selectedRow),
        });

        if (response.ok) {
          alert("Zapis uspješno ažuriran.");
          this.fetchData(); 
          this.closeModal();
        } else {
          console.error("Greška prilikom ažuriranja zapisa.");
        }
      } catch (error) {
        console.error("Greška:", error);
      }
    },

    closeModal() {
      this.showModal = false;
      this.selectedRow = null;
    },

    urlChooser(){
      if (
        this.selectedRow.hasOwnProperty("id_dogadaj") &&
        this.selectedRow.hasOwnProperty("id_organizator") &&
        this.selectedRow.hasOwnProperty("id_lokacija")
      ) {
        return `${this.apiEndpoint}/${this.selectedRow.id_dogadaj}/${this.selectedRow.id_organizator}/${this.selectedRow.id_lokacija}`;
      } 
      else if (this.selectedRow.hasOwnProperty("id_volonter") && this.selectedRow.hasOwnProperty("id_vjestina"))
      {
         return `${this.apiEndpoint}/${this.selectedRow.id_volonter}/${this.selectedRow.id_vjestina}`;
      } 
      
      else if ( this.selectedRow.hasOwnProperty("id_volonter") && this.selectedRow.hasOwnProperty("id_dogadaj"))
      {
         return `${this.apiEndpoint}/${this.selectedRow.id_volonter}/${this.selectedRow.id_dogadaj}`;
      }
    
      else 
      {
        return `${this.apiEndpoint}/${this.selectedRow.id}`;
      }
    }
  },
};
</script>

<style scoped>
.table-wrapper {
  padding: 2rem;
  text-align: center;
}

.start-title {
  font-size: 2rem;
  margin-bottom: 2rem;
  color: #ffffff;
}

.data-table {
  width: 80%;
  margin: 0 auto;
  border-collapse: collapse;
}

.data-table th,
.data-table td {
  border: 1px solid #ddd;
  padding: 12px;
  text-align: center;
}

.data-table th {
  background-color: #535bf2;
  color: white;
  font-weight: bold;
}

.data-table tr:nth-child(even) {
  background-color: #404252;
}

.data-table tr:hover {
  background-color: #3a8a96;
}

.data-table td {
  color: #ddd;
}

.table-wrapper {
  padding: 2rem;
}

.data-table th,
.data-table td {
  padding: 8px;
  text-align: center;
}

button {
  margin: 0 4px;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal-content {
  background: #333;
  padding: 2rem;
  border-radius: 16px;
  width: 400px;
  box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.2);
}

.modal-content h2 {
  margin-bottom: 1rem;
}

.form-group {
  margin-bottom: 1rem;
}

.modal-actions {
  display: flex;
  justify-content: space-between;
}

.modal-actions button {
  margin-top: 1rem;
}
</style>

