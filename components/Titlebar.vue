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
  <div data-tauri-drag-region class="navbar">
    <div></div>
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
.navbar {
  width: 100vw;
  position: fixed;
  height: 30px;
  display: flex;
  justify-content: space-between;
  background-color: #121212;
  z-index: 1000;
}

.window-controls {
  display: flex;
  flex-direction: row;

  img {
    width: 10px;
    height: 10px;
    stroke: #bdbdbd;
  }

  .button {
    background: none;
    border: none;
    outline: none;
    width: 46px;
    height: 30px;
    display: grid;
    place-items: center;
  }

  .button:hover {
    background-color: #ffffff24;
  }
}
</style>