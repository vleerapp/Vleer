<script setup>
// import { appWindow } from '@tauri-apps/api/window'

const os = ref('')

console.log("test")

os.value = await window.__TAURI__.core.invoke('plugin:utils|get_os')
if (os.value !== 'MacOS') appWindow.setDecorations(native_decorations)
</script>

<template>
  <div data-tauri-drag-region class="navbar">
    <div></div>
    <div class="window-controls">
      <button class="button minimize" @click="() => appWindow.minimize()">
        <img src="/minimize.svg" alt="">
      </button>
      <button class="button maximize" @click="() => appWindow.toggleMaximize()">
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