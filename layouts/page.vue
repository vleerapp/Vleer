<template>
  <sidebar />
  <div id="content" class="content animation" :class="minimized ? 'contentMinimized' : ''">
    <search />
    <slot />
  </div>
  <div class="bg"></div>
  <player />
</template>

<style>
.sidebar {
  position: fixed;
}

.content {
  position: absolute;
  top: 100px;
  left: 372px;
  width: calc(100% - 450px);
  padding-bottom: 100px;
}

.bg {
  width: 100%;
  height: 140px;
  position: fixed;
  background-color: var(--bg);
  bottom: 0;
  left: 350px;
  z-index: 4;
}

@media screen and (max-width: 1200px) {
  .content {
    left: 152px;
    width: calc(100% - 250px);
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

contents = JSON.parse(contents);

var minimized = contents.miniSidebar;
</script>
