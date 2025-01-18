
<template>
<main class="container">

  <div class ="highlightMain">
  <h1 class="start-title">Volonterium</h1>  
  <p>Volonterium je namijenjen kao aplikacija za upravljanje volontiranjem u sklopu projekta za predmet Baze Podataka 2</p>
  
  <p>Projektni plan i ideja za rješenje:</p>
  <ul class="projectList">
    <li>Naziv teme projekta: Upravljanje volonterskim događajima i resursima</li>
    <br>
    <li>Predviđeni Alati:</li>
    <li>1.	Konceptualno modeliranje ERA modela: ERDPlus <a href="https://erdplus.com/">[ErdPlus]</a></li>
    <li>2.	SUBP: Beekeper Studio, baza SQLite <a href="https://www.beekeeperstudio.io/">[Beekeper Studio]</a></li>
    <li>3.	Programski alat za sučelje: Tauri, Rust backend + Vue.js frontend <a href="https://v2.tauri.app/">[Tauri]</a></li>
  </ul>
  </div>

  </main>
</template>


<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.highlightMain{
  background-color: rgb(25, 81, 88);
  padding: 30px;
  margin: auto;
  border-radius: 30px;
  
}
a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
  margin-top: 2rem;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}


@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

.projectList{
  list-style-type: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 1rem;
}

</style>
