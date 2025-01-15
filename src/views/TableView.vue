<template>
    <div class="table-wrapper">
      <h1 class="start-title">{{ naslov }}</h1>
      <table class="data-table">
        <thead>
          <tr>
            <th v-for="(header, index) in headers" :key="index">{{ header }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, rowIndex) in data" :key="rowIndex">
            <td v-for="(value, key) in row" :key="key">{{ value }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>


<script>
export default {
  props: {
    apiEndpoint: {
      type: String,
      required: true,
    },
    naslov: {
      type: String,
      required: true,
    },
    headers: {
      type: Array,
      required: true,
    },
  },
  data() {
    return {
      data: [], 
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
</style>
