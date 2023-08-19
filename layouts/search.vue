<template>
  <sidebar />
  <div id="content" class="content animation" :class="minimized ? 'contentMinimized' : ''">
    <slot />
  </div>
  <div class="bg-player"></div>
  <player />
</template>

<style scoped>
body {
  position: relative;
}

.sidebar {
  position: fixed;
}

.bg-player {
  height: 140px;
  position: fixed;
  background-color: var(--bg);
  bottom: 0;
  border-radius: 12px;
  left: 350px;
  right: 10px;
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
