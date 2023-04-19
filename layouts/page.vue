<template>
  <sidebar />
  <div id="content" class="content animation" :class="minimized ? 'contentMinimized' : ''">
    <search />
    <slot />
  </div>
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

console.log(contents);

var minimized = contents.miniSidebar;
</script>
