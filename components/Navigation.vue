<script setup>
import { getCurrent } from "@tauri-apps/api/window";

const os = ref('')

os.value = await window.__TAURI__.core.invoke('get_os')
if (os.value == 'MacOS') {
  document.getElementById("window-controls").style.display = "false";
}
</script>

<template>
  <div data-tauri-drag-region class="navbar">
    <div></div>
    <div class="window-controls" id="window-controls">
      <button class="button minimize" @click="async () => await window.__TAURI__.appWindow.minimize()">
        <img src="/minimize.svg" alt="">
      </button>
      <button class="button maximize" @click="async () => await window.__TAURI__.appWindow.toggleMaximize()">
        <img src="/maximize.svg" alt="">
      </button>
      <button class="button close" @click="() => TauriWindow.getCurrent().close()">
        <img src="/close.svg" alt="">
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