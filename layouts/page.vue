<template>
  <sidebar />
  <div id="content" class="content animation" :class="minimized ? 'contentMinimized' : ''">
    <search />
    <slot />
  </div>
  <div class="bg-player"></div>
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
  padding-bottom: 150px;
}

.bg-player {
  height: 140px;
  position: fixed;
  background-color: var(--bg);
  bottom: 0;
  border-radius: 12px;
  left: 350px;
  right: 10px;
  z-index: 4;
  transition: 0.2s;
}

@media screen and (max-width: 1200px) {
  .content {
    left: 152px;
    width: calc(100% - 250px);
  }

  .bg {
    left: 130px;
  }

  .bg-player {
    transition: 0.2s;
    left: 124px !important;
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
