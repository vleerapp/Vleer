<template>
  <div class="sidebar animation" id="sidebar" :class="minimized ? 'minimized' : ''">
    <div class="minibar">
      <nuxt-link class="settings" to="/settings">
        <img src="/svg/linear/setting-2.svg" width="18" height="18" />
      </nuxt-link>
    </div>
    <div id="sidenav_tabs">
      <div class="tab">
        <nuxt-link to="/" class="tab-link">
          <div
            style="--img: url('/svg/linear/home-2.svg')"
            class="tab_icon home_icon"
          ></div>
          <span class="tab_text">Home</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/songs" class="tab-link">
          <div
            style="--img: url('/svg/linear/music.svg')"
            class="tab_icon songs_icon"
          ></div>
          <span class="tab_text">Songs</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/playlists" class="tab-link">
          <div
            style="--img: url('/svg/linear/note.svg')"
            class="tab_icon playlists_icon"
          ></div>
          <span class="tab_text">Playlists</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/library" class="tab-link">
          <div
            style="--img: url(/svg/linear/video-square.svg)"
            class="tab_icon library_icon"
          ></div>
          <span class="tab_text">Library</span>
        </nuxt-link>
      </div>
    </div>
    <div
      id="minimizer"
      style="--arrow: url('/svg/linear/arrow-close.svg')"
      v-on:click="toggleSidebar"
      class="minimizeSidebar animation"
      :class="minimized ? 'mini' : ''"
      alt=""
    ></div>
  </div>
</template>

<script setup lang="ts">
import {
  writeTextFile,
  createDir,
  BaseDirectory,
  exists,
  readTextFile,
} from "@tauri-apps/api/fs";

var contents = await readTextFile("config.json", {
  dir: BaseDirectory.AppConfig,
});

contents = JSON.parse(contents);

var minimized = contents.miniSidebar;

async function toggleSidebar() {
  var element = document.getElementById("sidebar");
  var content = document.getElementById("content");
  var minimizer = document.getElementById("minimizer");
  if (element && content && minimizer) {
    element.classList.toggle("minimized");
    minimizer.classList.toggle("mini");
    content.classList.toggle("contentMinimized");

    var contents = await readTextFile("config.json", {
      dir: BaseDirectory.AppConfig,
    });

    var parsedContents = JSON.parse(contents);

    parsedContents["miniSidebar"] = element.classList.contains("minimized");

    await saveFile(parsedContents);
  }
}

async function saveFile(contents: JSON) {
  try {
    if (!(await exists("", { dir: BaseDirectory.AppConfig }))) {
      await createDir("", {
        dir: BaseDirectory.AppConfig,
        recursive: true,
      });
    }

    await writeTextFile("config.json", JSON.stringify(contents), {
      dir: BaseDirectory.AppConfig,
    });
  } catch (err) {
    console.log(err);
  }
}
</script>

<style>
@import "~/css/sidebar.css";
@import "~/css/media.css";
</style>
