<script lang="ts">
  import Greet from "$lib/Greet.svelte";
  import type { PageData } from "./$types";

  export let data: PageData;

  let count = 0

  console.log(data);
</script>

<h1>Welcome to Tauri!</h1>

<div class="row">
  <a href="https://vitejs.dev" target="_blank">
    <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
  </a>
  <a href="https://tauri.app" target="_blank">
    <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
  </a>
  <a href="https://kit.svelte.dev" target="_blank">
    <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
  </a>
</div>

<p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>

{count}
<button on:click={() => {count++}}></button>

{#each data.genres as genre}
<p>{genre.label}</p>
  
{/each}

<div class="row">
  {data.hest}
  <Greet />
</div>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
