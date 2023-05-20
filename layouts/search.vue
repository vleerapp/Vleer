<template>
  <sidebar />
  <div id="content" class="content animation" :class="minimized ? 'contentMinimized' : ''">
    <slot />
  </div>
  <div class="bg"></div>
</template>

<style scoped>
body {
  position: relative;
}

.sidebar {
  position: fixed;
}

.content {
  position: absolute;
  left: 372px;
  transition: .2s;
  top: 110px;
}

@media screen and (max-width: 1200px) {
  .content {
    left: 152px;
  }
}
</style>

<script setup lang="ts">
import {
  BaseDirectory,
  readTextFile,
} from "@tauri-apps/api/fs";

var contents = await readTextFile("config.json", {
  dir: BaseDirectory.AppConfig,
});

var config = JSON.parse(contents);

var minimized = config.miniSidebar;
</script>
