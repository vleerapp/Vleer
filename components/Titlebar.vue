<script setup>
import { ref, onMounted } from 'vue';
import { getCurrent } from "@tauri-apps/api/window";

const isMaximized = ref(false);

onMounted(async () => {
  const currentWindow = await getCurrent();
  isMaximized.value = await currentWindow.isMaximized();

  if (await window.__TAURI__.core.invoke('get_os') == 'MacOS') {
    const controls = document.getElementById("window-controls")
    controls.style.display = "none";
  }

  currentWindow.listen('tauri://resize', async () => {
    isMaximized.value = await currentWindow.isMaximized();
  });
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
        <img v-if="!isMaximized" src="/maximize.svg" alt="maximize">
        <img v-else src="/restore.svg" alt="restore">
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