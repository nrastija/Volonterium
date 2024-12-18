<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div>
    <nav>
      <router-link class="router" to="/">Početna</router-link>
      <router-link class="router" to="/table">Tablica</router-link>
    </nav>
    <!-- Ovdje će se prikazivati komponente iz routera -->
    <router-view></router-view>
  </div>
 </template>

<style scoped>
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
