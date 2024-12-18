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
      <router-link to="/">Početna</router-link>
      <router-link to="/table">Tablica</router-link>
    </nav>
    <!-- Ovdje će se prikazivati komponente iz routera -->
    <router-view></router-view>
  </div>
 </template>