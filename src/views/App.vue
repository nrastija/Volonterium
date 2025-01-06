

<template>
  <div>
    <h1>{{ pageTitle }}</h1>
    <nav>
      <router-link class="router" to="/">Početna</router-link>
      <router-link class="router" to="/table">Tablica</router-link>
    </nav>
    <router-view></router-view>
  </div>
 </template>

<script>
export default {
  computed: {
    pageTitle() {
      return this.$route.meta?.title || "Dobrodošli u Volonterium!";
    },
  },
};
</script>

<style scoped>

@keyframes neon-pulse {
  0%, 100% {
    text-shadow: 
      0 0 5px #32a9b9, 
      0 0 10px #32a9b9, 
      0 0 20px #32a9b9, 
      0 0 40px #67dfef, 
      0 0 80px #67dfef, 
      0 0 90px #67dfef, 
      0 0 100px #32a9b9, 
      0 0 150px #67dfef;
  }
  50% {
    text-shadow: 
      0 0 2px #32a9b9, 
      0 0 4px #32a9b9, 
      0 0 8px #32a9b9, 
      0 0 16px #67dfef, 
      0 0 32px #67dfef, 
      0 0 36px #67dfef, 
      0 0 40px #67dfef, 
      0 0 75px #67dfef;
  }
}

h1{
  color: rgb(255, 255, 255);
  margin: 20px 0;
  animation: neon-pulse 3s infinite
}

nav {
  display: flex;
  flex-direction: row;
  justify-content: center;
  gap: 1rem;
  margin-top: 2rem;
  background-color: #333;
  padding: 10px;
}

.router {
  text-decoration: inherit;
  font-size: 1.3rem;
  color: white;
  transition:  0.75s; 
}

.router:hover {
  color: #535bf2 ; /* Change background color on hover */
  transform: scale(1.05); 
}

</style>
