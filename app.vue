<template>
  <div>
    <titlebar />
    <NuxtPage />
  </div>
  <div id="app"></div>
  <audio id="media"></audio>
  <div id="overlay">
    <div class="allowDrop" style="
                  color: var(--succeed);
                  place-items: center;
                  font-size: 20px;
                  font-weight: bold;
                  padding: 10px;
                ">
      <svg id="svg1" xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 24 24" fill="none"
        stroke="#66ff66" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
        <polyline points="7 10 12 15 17 10"></polyline>
        <line x1="12" y1="15" x2="12" y2="3"></line>
      </svg>
      <div>Import</div>
    </div>
    <div class="deniDrop" style="
                    color: var(--fail);
                    place-items: center;
                    font-size: 20px;
                    font-weight: bold;
                    padding: 10px;
                  ">
      <svg id="svg2" xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 24 24" fill="none"
        stroke="#ff6666" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
      <div>Only .mp3</div>
    </div>
  </div>
</template>

<style>
:root {
  /*=======COLORS===========*/
  --bg: #000000;
  --sidebar: #0F0E14;
  --selected: #1B1A26;
  --accent: #7b00ff;
  --accenth: #6c00e0;
  --element: #1c1b21;
  --elementh: #232229;
  --tx: #ffffff;
  --unselected: #8D8A9C;
  --search: #1e1d23;
  --border: #26252b;
  --fail: #ff6666;
  --succeed: #66ff66;

  /* backup */
  /* --bg: #16151a;
  --sidebar: #19181d;
  --accent: #7b00ff;
  --accenth: #6c00e0;
  --element: #1c1b21;
  --elementh: #232229;
  --tx: #ffffff;
  --dtx: #85868b;
  --search: #1e1d23;
  --border: #26252b;
  --fail: #ff6666;
  --succeed: #66ff66; */
}

@font-face {
  font-family: Inter;
  src: url("/Inter.woff2") format("woff2");
  font-display: swap;
}

* {
  scroll-behavior: smooth;
}

body {
  overflow-x: hidden;
  background-color: transparent !important;
  position: relative;
}

html{
  overflow: hidden !important;
}

::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
  margin: 3px;
  margin-top: 45px;
}

::-webkit-scrollbar-thumb {
  background: var(--sidebar);
  border-radius: 12px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--search);
}

#app {
  overflow: hidden !important;
  background-color: var(--bg);
  position: fixed;
  height: 100vh;
  width: 100vw;
  border-radius: 24px !important;
  z-index: -1;
}

#overlay {
  --overlayColor: var(--succeed);
  pointer-events: none;
  opacity: 0;
  transition: 0.5s;
  position: fixed;
  height: 100vh;
  width: 100vw;
  border-radius: 24px;
  z-index: 10;
  border: 5px solid var(--overlayColor);
  display: grid;
  place-items: center;
  backdrop-filter: brightness(20%);
}

#svg1,
#svg2 {
  scale: 0.5;
  transition: 0.5s;
  transition-timing-function: cubic-bezier(1, -0.53, 0.405, 1.425);
}

#overlay.red {
  --overlayColor: var(--fail) !important;
}

.deniDrop {
  display: none;
}

#overlay>.allowDrop {
  display: grid;
}

#overlay>.deniDrop {
  display: none;
}

#overlay.red>.allowDrop {
  display: none;
}

#overlay.red>.deniDrop {
  display: grid;
}

#top {
  position: absolute;
  top: -200px;
}

.gotoTop {
  transition: 0.5s;
  position: fixed;
  width: 30px;
  height: 30px;
  background-repeat: no-repeat;
  background-size: 20px;
  background-position: 50% 50%;
  bottom: 10px;
  right: 20px;
  background-image: url(/svg/bold/arrow-up.svg);
  border-radius: 6px;
  z-index: 3;
}

.gotoTop:hover {
  background-color: var(--search);
}
</style>

<script setup>
import { appWindow } from "@tauri-apps/api/window";
import {
  copyFile,
  BaseDirectory,
  createDir,
  exists,
  writeTextFile,
  readTextFile,
} from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api";
import { MusicHandler } from '/musicHandler'
import { platform } from '@tauri-apps/api/os';
const platformName = await platform();

var contents = await readTextFile("config.json", {
  dir: BaseDirectory.AppConfig,
});

var parsedContents = JSON.parse(contents);

var musicHandler = MusicHandler.getInstance();
musicHandler.volume(parsedContents["volume"]);

async function checkForDefaultFilesInAppData() {
  if (!(await exists("", { dir: BaseDirectory.AppConfig }))) {
    await createDir("", {
      dir: BaseDirectory.AppConfig,
      recursive: true,
    });
  }

  if (!(await exists("config.json", { dir: BaseDirectory.AppConfig }))) {
    await writeTextFile("config.json", "{}", {
      dir: BaseDirectory.AppConfig,
    });
  }
}

await checkForDefaultFilesInAppData();

onMounted(async () => {
  try {
    var isma = await appWindow.isMaximized();
    if (isma) {
      document.getElementById("app").style.borderRadius = "0";
    } else {
      document.getElementById("app").style.borderRadius = "6px";
    }
  }
  catch (e) {
    console.log("Error, while trying to remove border radius on maximize: "+e);
  }

  window.addEventListener("keydown", function (event) {
    if (
      event.code === "Space" &&
      !document.getElementById("searchBar").activeElement
    ) {
      event.preventDefault();
      musicHandler.pauseplay();
    }
  });
})

if (platformName == "win32"|| platformName == "win64") {
  appWindow.onResized(async () => {
    try {
      var isma = await appWindow.isMaximized();
      if (isma) {
        document.getElementById("app").style.borderRadius = "0";
      } else {
        document.getElementById("app").style.borderRadius = "6px";
      }
    }
    catch (e) {
      console.log("Error, while trying to remove border radius on maximize: "+e);
    }
  });
}

document.body.addEventListener("scroll", () => {
  if (document.body.scrollTop < 800) {
    document.getElementById("gotoTop").style.opacity = "0";
    document.getElementById("gotoTop").style.pointerEvents = "none";
  } else {
    document.getElementById("gotoTop").style.opacity = "1";
    document.getElementById("gotoTop").style.pointerEvents = "all";
  }
});

appWindow.onFileDropEvent(async (event) => {
  var overlay = document.getElementById("overlay");

  if (event.payload.type === "cancel") {
    overlay.style.opacity = "0";
    document.getElementById("svg1").style.scale = ".5";
    document.getElementById("svg2").style.scale = ".5";
    return;
  } else if (event.payload.paths.length === 0) {
    overlay.style.opacity = "0";
    document.getElementById("svg1").style.scale = ".5";
    document.getElementById("svg2").style.scale = ".5";
    return;
  } else if (event.payload.type === "hover") {
    overlay.style.opacity = "1";
    document.getElementById("svg1").style.scale = "1";
    document.getElementById("svg2").style.scale = "1";
    if (!event.payload.paths.every((path) => path.endsWith(".mp3"))) {
      overlay.classList.add("red");
    } else {
      overlay.classList.remove("red");
    }
  } else if (event.payload.type === "drop") {
    overlay.style.opacity = "0";
    document.getElementById("svg1").style.scale = ".5";
    document.getElementById("svg2").style.scale = ".5";
    if (!event.payload.paths.every((path) => path.endsWith(".mp3"))) return;

    await checkForDefaultFiles();

    var songNames = [];
    var index = 0;
    for (const item of event.payload.paths) {
      var musicName = item.split("\\");
      musicName = musicName[musicName.length - 1];
      if (
        await exists(`savedMusic/${musicName}`, { dir: BaseDirectory.Audio })
      ) {
        continue;
      }

      const musicmetadata = await invoke("get_metadata", { source: item });

      await copyFile(item, `savedMusic/${musicName}`, {
        dir: BaseDirectory.Audio,
      });

      songNames.push(musicName);

      const { source, title, artist, album, album_art, duration } =
        musicmetadata;

      const currentDate = new Date();
      const year = currentDate.getFullYear();
      const month = String(currentDate.getMonth() + 1).padStart(2, "0");
      const day = String(currentDate.getDate()).padStart(2, "0");
      const hours = String(currentDate.getHours()).padStart(2, "0");
      const minutes = String(currentDate.getMinutes()).padStart(2, "0");
      const seconds = String(currentDate.getSeconds()).padStart(2, "0");
      const formattedDate = `${year}-${month}-${day}-${hours}-${minutes}-${seconds}`;

      var musicData = {
        trackName: title,
        artistName: artist,
        album,
        imageURL: album_art,
        index: index,
        added: formattedDate,
        trackTimeMillis: duration,
        audioFile: `${musicName.slice(0, -4)}`,
        playlist: "",
      };

      await writeTextFile(
        `savedMusic/${musicName}.json`,
        JSON.stringify(musicData),
        {
          dir: BaseDirectory.Audio,
        }
      );
      index++;
    }
    await addPaths(songNames);
    location.reload();
  }
});

async function checkForDefaultFiles() {
  if (!(await exists("savedMusic/", { dir: BaseDirectory.Audio }))) {
    await createDir("savedMusic/", {
      dir: BaseDirectory.Audio,
    });
  }
  if (!(await exists("savedMusic/_all.json", { dir: BaseDirectory.Audio }))) {
    var all = { all: [], counter: 0 };
    await writeTextFile(`savedMusic/_all.json`, JSON.stringify(all), {
      dir: BaseDirectory.Audio,
    });
  }
}
async function addPaths(songNames) {
  var content = await readTextFile(`savedMusic/_all.json`, {
    dir: BaseDirectory.Audio,
  });
  content = JSON.parse(content);

  content.all = content.all.concat(songNames);

  await writeTextFile(`savedMusic/_all.json`, JSON.stringify(content), {
    dir: BaseDirectory.Audio,
  });
}
</script>
