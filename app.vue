<template>
  <div>
    <NuxtLayout>
      <titlebar />
      <NuxtPage />
    </NuxtLayout>
  </div>
  <div id="app"></div>
  <div id="overlay">
    <svg
      class="allowDrop"
      xmlns="http://www.w3.org/2000/svg"
      width="100"
      height="100"
      viewBox="0 0 24 24"
      fill="none"
      stroke="#66ff66"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
      <polyline points="7 10 12 15 17 10"></polyline>
      <line x1="12" y1="15" x2="12" y2="3"></line>
    </svg>
    <svg
      class="deniDrop"
      xmlns="http://www.w3.org/2000/svg"
      width="100"
      height="100"
      viewBox="0 0 24 24"
      fill="none"
      stroke="#ff6666"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <line x1="18" y1="6" x2="6" y2="18"></line>
      <line x1="6" y1="6" x2="18" y2="18"></line>
    </svg>
  </div>
</template>

<style>
:root {
  /*=======COLORS===========*/
  --bg: #16151a;
  --sidebar: #19181d;
  --e: #1c1b21;
  --dtx: #85868b;
  --tx: #ffffff;
  --t: #e06c75;
  --search: #1e1d23;
  --border: #26252b;
  --accent: #7b00ff;
  --fail: #ff6666;
  --succeed: #66ff66;
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
  background-color: var(--bg);
  position: fixed;
  height: 100vh;
  width: 100vw;
  border-radius: 10px;
  z-index: -1;
}

#overlay {
  --overlayColor: var(--succeed);
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.5s;
  position: fixed;
  margin: 10px;
  height: calc(100vh - 20px);
  width: calc(100vw - 20px);
  border-radius: 5px;
  z-index: 10;
  border: 5px solid var(--overlayColor);
  display: grid;
  place-items: center;
}
#overlay.red {
  --overlayColor: var(--fail) !important;
}

.deniDrop {
  display: none;
}

#overlay.red > .allowDrop {
  display: none;
}

#overlay.red > .deniDrop {
  display: block;
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
  Dir,
} from "@tauri-apps/api/fs";
import { metadata } from "tauri-plugin-fs-extra-api";

/* import { getMatches } from '@tauri-apps/api/cli';

const matches = await getMatches();
if (matches.subcommand?.name === 'run') {
  const args = matches.subcommand?.matches.args
  if ('test' in args) {
    console.log('%c--test was executed', 'color: green');
  }
} else {
  const args = matches.args
} */

appWindow.onResized(async () => {
  var isma = await appWindow.isMaximized();
  if (isma) {
    document.getElementById("app").style.borderRadius = "0";
  } else {
    document.getElementById("app").style.borderRadius = "6px";
  }
});

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
  if (event.payload.type === "hover") {
    var musicName = event.payload.paths[0].split("\\");
    var musicEnd = musicName[musicName.length - 1].slice(-3);
    musicName = musicName[musicName.length - 1].slice(0, -4);
    overlay.classList.remove("red");
    overlay.style.opacity = "1";
    if (musicEnd != "mp3") {
      overlay.classList.add("red");
    }
    console.log(BaseDirectory.Audio);
  } else if (event.payload.type === "drop") {
    overlay.style.opacity = "0";
    event.payload.paths.forEach(async (item, index) => {
      var musicName = item.split("\\");
      musicName = musicName[musicName.length - 1];
      if (
          !(await exists(musicName, { dir: BaseDirectory.Audio }))
        ) {
          return;
        }
      if (musicEnd != "mp3") {
        if (!(await exists("savedMusic/", { dir: BaseDirectory.Audio }))) {
          await createDir("savedMusic/", {
            dir: BaseDirectory.Audio,
            recursive: true,
          });
        }
        if (
          !(await exists("savedMusic/_all.json", { dir: BaseDirectory.Audio }))
        ) {
          var all = {
            "all": []
          };
          await writeTextFile(
            `savedMusic/_all.json`,
            JSON.stringify(all),
            {
              dir: BaseDirectory.Audio,
            }
          );
        }
        await copyFile(item, `savedMusic/${musicName}`, {
          dir: BaseDirectory.Audio,
        });

        var allFile = await readTextFile(`savedMusic/_all.json`, {
          dir: BaseDirectory.Audio,
        });
        allFile = JSON.parse(allFile);

        allFile.all.push(musicName)

        await writeTextFile(
          `savedMusic/_all.json`,
          JSON.stringify(allFile),
          {
            dir: BaseDirectory.Audio,
          }
        );

        var md = await metadata(item);
        console.log(md)

        var musicData = {
          trackName: `${musicName.slice(0, -4)}`,
          artistName: "",
          kind: "",
          image: "",
          releaseDate: "",
          trackExplicitness: "",
          trackTimeMillis: "",
          country: "",
          audioFile: musicName,
          playlist: "",
        };
        await writeTextFile(
          `savedMusic/${musicName}.json`,
          JSON.stringify(musicData),
          {
            dir: BaseDirectory.Audio,
          }
        );
        var contents = await readTextFile(`savedMusic/${musicName}.json`, {
          dir: BaseDirectory.Audio,
        });
        contents = JSON.parse(contents);
        console.log(contents);
      }
    });
  } else {
    overlay.style.opacity = "0";
  }
});
</script>
