<template>
  <div class="sidebar animation" id="sidebar" :class="minimized ? 'minimized' : ''">
    <div class="minibar">
      <img id="avatar-img" src="/empty.png" alt="avatar" />
      <nuxt-link class="settings" to="/settings">
        <img src="/svg/linear/setting-2.svg" width="18" height="18" />
      </nuxt-link>
    </div>
    <div id="sidenav_tabs">
      <div class="tab">
        <nuxt-link to="/" class="tab-link">
          <div style="--img: url('/svg/linear/home-2.svg')" class="tab_icon home_icon"></div>
          <span class="tab_text">Home</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/songs" class="tab-link">
          <div style="--img: url('/svg/linear/music.svg')" class="tab_icon songs_icon"></div>
          <span class="tab_text">Songs</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/playlists" class="tab-link">
          <div style="--img: url('/svg/linear/note.svg')" class="tab_icon playlists_icon"></div>
          <span class="tab_text">Playlists</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/library" class="tab-link">
          <div style="--img: url(/svg/linear/video-square.svg)" class="tab_icon library_icon"></div>
          <span class="tab_text">Library</span>
        </nuxt-link>
      </div>
    </div>
    <div id="minimizer" style="--arrow: url('/svg/linear/arrow-close.svg')" v-on:click="toggleSidebar"
      class="minimizeSidebar animation" :class="minimized ? 'mini' : ''" alt=""></div>
  </div>
</template>

<script setup lang="ts">
import {
  writeTextFile,
  createDir,
  BaseDirectory,
  exists,
  readTextFile,
  readBinaryFile,
  copyFile,
} from "@tauri-apps/api/fs";
import { appConfigDir } from '@tauri-apps/api/path';
import { convertFileSrc } from "@tauri-apps/api/tauri";

interface Contents {
  avatarPath: string;
  miniSidebar: boolean;
}

var contents = await readTextFile('config.json', {
  dir: BaseDirectory.AppConfig,
});

var parsedContents = <Contents>JSON.parse(contents);

var minimized = parsedContents["miniSidebar"];

async function loadImage() {
  try {
    const image = document.getElementById("avatar-img") as HTMLImageElement;

    const img_src = await readBinaryFile('avatar.png', { dir: BaseDirectory.AppConfig });

    const binaryData = new Uint8Array(img_src);

    const blob = new Blob([binaryData], { type: 'image/png' });

    const imageUrl = URL.createObjectURL(blob);

    image.src = imageUrl;
  } catch (error) {
    console.error("Error loading image:", error);
  }
}

onMounted(async () => {
  loadImage();
})

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

    var parsedContents = <Contents>JSON.parse(contents);

    var minimized = parsedContents["miniSidebar"];

    parsedContents["miniSidebar"] = element.classList.contains("minimized");

    await saveFile(JSON.parse(JSON.stringify(parsedContents)));
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

<script lang="ts">
</script>

<style>
@import "~/css/sidebar.css";
@import "~/css/media.css";
</style>
