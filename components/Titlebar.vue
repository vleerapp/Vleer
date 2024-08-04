<script setup>
import { getCurrentWindow } from "@tauri-apps/api/window";
import { platform } from '@tauri-apps/plugin-os';

const isMaximized = ref(false);

onMounted(async () => {
  const currentWindow = await getCurrentWindow();
  isMaximized.value = await currentWindow.isMaximized();

  if (await platform() == 'macos') {
    const controls = document.getElementById("window-controls")
    controls.style.display = "none";
  }

  currentWindow.listen('tauri://resize', async () => {
    isMaximized.value = await currentWindow.isMaximized();
  });
})

</script>

<template>
  <div class="titlebar">
    <div class="drag-region" data-tauri-drag-region></div>
    <div class="window-controls" id="window-controls">
      <button class="button minimize" @click="async () => getCurrentWindow().minimize()">
        <img src="/minimize.svg" alt="minimize">
      </button>
      <button class="button maximize" @click="async () => getCurrentWindow().toggleMaximize()">
        <img v-if="!isMaximized" src="/maximize.svg" alt="maximize">
        <img v-else src="/restore.svg" alt="restore">
      </button>
      <button class="button close" @click="() => getCurrentWindow().close()">
        <img src="/close.svg" alt="close">
      </button>
    </div>
  </div>
</template>


<style lang="scss">
@use '~/assets/styles/components/titlebar';
</style>