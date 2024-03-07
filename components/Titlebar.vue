<script setup>
import { onMounted } from 'vue';
import { getCurrent } from "@tauri-apps/api/window";

onMounted(async () => {
  if (await window.__TAURI__.core.invoke('get_os') == 'MacOS') {
    const controls = document.getElementById("window-controls")
    controls.style.display = "none";
  }
})

</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <p class="titlebar-text">Vleer</p>
    <div class="window-controls" id="window-controls">
      <button class="button minimize" @click="async () => getCurrent().minimize()">
        <img src="/minimize.svg" alt="minimize">
      </button>
      <button class="button maximize" @click="async () => getCurrent().toggleMaximize()">
        <img src="/maximize.svg" alt="maximize">
      </button>
      <button class="button close" @click="() => getCurrent().close()">
        <img src="/close.svg" alt="close">
      </button>
    </div>
  </div>
</template>


<style lang="scss">
@use '~/assets/styles/titlebar';
</style>