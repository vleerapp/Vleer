<template>
  <div>
    <NuxtLayout>
      <titlebar />
      <NuxtPage />
    </NuxtLayout>
  </div>
  <div id="app"></div>
  <div id="overlay">
    <div
      class="allowDrop"
      style="
        color: var(--succeed);
        place-items: center;
        font-size: 20px;
        font-weight: bold;
      "
    >
      <svg
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
      <div>Import</div>
    </div>
    <div
      class="deniDrop"
      style="
        color: var(--fail);
        place-items: center;
        font-size: 20px;
        font-weight: bold;
      "
    >
      <svg
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
      <div>Only .mp3</div>
    </div>
  </div>
</template>

<style>
:root {
  /*=======COLORS===========*/
  --bg: #16151a;
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

#overlay > .allowDrop {
  display: grid;
}

#overlay > .deniDrop {
  display: none;
}

#overlay.red > .allowDrop {
  display: none;
}

#overlay.red > .deniDrop {
  display: grid;
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

const itunes = "https://itunes.apple.com/search?";
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

  if (event.payload.type === "cancel") {
    overlay.style.opacity = "0";
    return;
  } else if (event.payload.type === "hover") {
    overlay.style.opacity = "1";
    if (!event.payload.paths.every((path) => path.endsWith(".mp3"))) {
      overlay.classList.add("red");
    } else {
      overlay.classList.remove("red");
    }
  } else if (event.payload.type === "drop") {
    overlay.style.opacity = "0";
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

      await copyFile(item, `savedMusic/${musicName}`, {
        dir: BaseDirectory.Audio,
      });

      songNames.push(musicName);

      const currentDate = new Date();
      const year = currentDate.getFullYear();
      const month = String(currentDate.getMonth() + 1).padStart(2, "0");
      const day = String(currentDate.getDate()).padStart(2, "0");
      const hours = String(currentDate.getHours()).padStart(2, "0");
      const minutes = String(currentDate.getMinutes()).padStart(2, "0");
      const seconds = String(currentDate.getSeconds()).padStart(2, "0");
      const formattedDate = `${year}-${month}-${day}-${hours}-${minutes}-${seconds}`;

      var musicData = {
        trackName: `${musicName.slice(0, -4)}`,
        artistName: "",
        imageURL: "",
        index: index,
        added: formattedDate,
        trackTimeMillis: "",
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
      index++
    }
    await addPaths(songNames);
  }
});

async function checkForDefaultFiles() {
  if (!(await exists("savedMusic/", { dir: BaseDirectory.Audio }))) {
    await createDir("savedMusic/", {
      dir: BaseDirectory.Audio,
    });
  }
  if (!(await exists("savedMusic/_all.json", { dir: BaseDirectory.Audio }))) {
    var all = [];
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

  content = content.concat(songNames);

  console.log(content);

  await writeTextFile(`savedMusic/_all.json`, JSON.stringify(content), {
    dir: BaseDirectory.Audio,
  });
}

class Song {
  constructor(json) {
    if ("results" in json) {
      try {
        json = json["results"][0];
      } catch { }
    }
    this.kind = json["kind"];
    this.artistName = json["artistName"];
    this.collectionName = json["collectionName"];
    this.trackName = json["trackName"];
    this.artistViewUrl = json["artistViewUrl"];
    this.collectionViewUrl = json["collectionViewUrl"];
    this.trackViewUrl = json["trackViewUrl"];
    this.image = json["artworkUrl100"];
    this.releaseDate = json["releaseDate"];
    this.collectionExplicitness =
      json["collectionExplicitness"] == "notExplicit" ? false : true;
    this.trackExplicitness =
      json["trackExplicitness"] == "notExplicit" ? false : true;
    this.discCount = json["discCount"];
    this.discNumber = json["discNumber"];
    this.trackCount = json["trackCount"];
    this.trackTimeMillis = json["trackTimeMillis"];
    this.country = json["country"];
    this.primaryGenreName = json["primaryGenreName"];
    this.isStreamable = json["isStreamable"].toString();
    this.artistId = json["artistId"];
    this.collectionId = json["collectionId"];
    this.trackId = json["trackId"];
  }

  getImage() {
    return this.image;
  }

  getName() {
    return this.trackName;
  }

  getArtistName() {
    return this.artistName;
  }

  getCountry() {
    return this.country;
  }

  getTrackViewUrl() {
    return this.trackViewUrl;
  }

  isStreamable() {
    return this.isStreamable;
  }

  getCollectionName() {
    return this.collectionName;
  }

  getResizedImage(size) {
    return resizeImage(this.image, size);
  }

  getIds() {
    return [this.trackId, this.collectionId, this.artistId];
  }

  getLength() {
    return this.trackTimeMillis;
  }

  getLengthNormal() {
    const totalSeconds = Math.floor(this.trackTimeMillis / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes}:${seconds.toString().padStart(2, "0")}`;
  }

  isExplicit() {
    return this.trackExplicitness;
  }

  searchForSongName(country = "CH", limit = 50, explicit = true) {
    return get(this.trackName, country, limit, explicit);
  }

  videoURL() { }
}

function resizeImage(url, size) {
  url = url.replace("100x100bb.jpg", `${size}x${size}bb.jpg`);
  url = url.replace("60x60bb.jpg", `${size}x${size}bb.jpg`);
  url = url.replace("30x30bb.jpg", `${size}x${size}bb.jpg`);
  return url;
}

function get(term, country = "CH", limit = 50, explicit = true) {
  term = term.replace(/\s+/g, '+');

  const apiUrl = `${itunes}term=${term}&media=music&entity=song&country=${country}&limit=${limit}&explicit=${explicit ? "Yes" : "No"
    }&attribute=genreIndex`;


  return fetch(apiUrl)
    .then((response) => response.json())
    .then((data) => {
      const songList = [];
      data.results.forEach((item) => {
        var song = new Song(item);
        songList.push(song);
      });
      return songList;
    });
}

</script>
