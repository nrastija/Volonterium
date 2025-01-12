<template>
    <div class="menu-container">
      <h1 class="start-title">Tablice s jednim primarnim ključem</h1>
      <div class="dropdown">
        <button class="dropdown-btn">Tablice</button>
        <div class="dropdown-content">

        </div>
      </div>
    </div>

    <div class="menu-container">
      <h1 class="start-title">Tablice s dvokomponentnim primaranim ključem</h1>
      <div class="dropdown">
        <button class="dropdown-btn">Tablice</button>
        <div class="dropdown-content">

        </div>
      </div>
    </div>

    <div class="menu-container">
      <h1 class="start-title">Tablice s troduplim primarnim ključem</h1>
      <div class="dropdown">
        <button class="dropdown-btn">Tablice</button>
        <div class="dropdown-content">

        </div>
      </div>
    </div>
</template>

<style scoped>
  .menu-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    font-family: Arial, sans-serif;
    margin: 2rem auto;
    margin-top: 3rem;
    background-color: rgb(2, 104, 117);
    padding: 50px;
    border-radius: 30px;
    max-width: 50%;
  }

  .dropdown {
    margin-top: 2rem;
    position: relative;
    display: inline-block;
  }
  
  .dropdown-btn {
    background-color: #535bf2;
    color: white;
    padding: 0.8rem 2rem;
    font-size: 1.2rem;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: background-color 0.3s ease, transform 0.2s ease;
  }
  
  .dropdown-btn:hover {
    background-color: #4042b8;
    transform: scale(1.05);
  }
  
  .dropdown-content {
    position: absolute;
    background-color: white;
    min-width: 200px;
    box-shadow: 0px 8px 16px rgba(0, 0, 0, 0.2);
    border-radius: 8px;
    z-index: 1;
    margin-top: 10px;

    opacity: 0; 
    visibility: hidden; 
    transition: opacity 0.5s ease, visibility 0.5s ease;
  }

  .dropdown:hover .dropdown-content {
    opacity: 1;
    visibility: visible; 
  }


  .dropdown-item {
    color: #535bf2;
    padding: 12px 16px;
    text-decoration: none;
    display: block;
    font-size: 1rem;
    font-weight: bold;
    border-bottom: 1px solid #f0f0f0;
    transition: background-color 0.2s, color 0.2s;
  }
  
  .dropdown-item:last-child {
    border-bottom: none;
  }
  
  .dropdown-item:hover {
    background-color: #f0f0f0;
    color: #4042b8;
  }
  

  </style>

<style>
.input-form{
    margin: 0 auto;
    width: 40%;
    padding: 20px;
    border: 1px solid #ccc;
    border-radius: 10px;
    margin-top: 20px;

    background-color: rgb(25, 81, 88);
    border-radius: 30px;

    display: flex;
    flex-direction: column;
    text-align: left;
    gap: 1rem;
}

.start-title{
    margin-top: 3rem;
    text-align: center;
}

.form-group{
    margin: 10px 0;
    flex-direction: column;
    gap: 0.5rem;
}

.form-group input {
    padding: 10px;
    font-size: 1rem;
    border: 1px solid #ccc;
    border-radius: 5px;
    width: 100%;
    box-sizing: border-box;
}

button {
    color: white;
    padding: 10px 20px;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    font-size: 1rem;
    transition: background-color 0.3s ease;
    text-align: center;
}

button:hover {
    background-color: #444;
}

</style>
  