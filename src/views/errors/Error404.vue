<template>
    <div class="error-page">
      <h1>404 - Stranica nije pronađena</h1>
      <p>Žao nam je, ali tražena stranica ne postoji.</p>
      <button @click="goHome">Vrati se na početnu</button>
    </div>
  </template>
  
  <script>
  export default {
    name: "Error",
    methods: {
      goHome() {
        this.$router.push("/");
      },
    },
  };
  </script>
  
  <style scoped>
  .error-page {
    text-align: center;
    margin: 50px auto;
    padding: 20px;
    max-width: 600px;
    background-color: #f8f9fa;
    border: 1px solid #ddd;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  .error-page h1 {
    font-size: 2rem;
    color: #dc3545;
  }
  
  .error-page p {
    font-size: 1.2rem;
    color: #6c757d;
  }
  
  .error-page button {
    padding: 10px 20px;
    font-size: 1rem;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    margin-top: 20px;
  }
  
  .error-page button:hover {
    background-color: #0056b3;
  }
  </style>
  