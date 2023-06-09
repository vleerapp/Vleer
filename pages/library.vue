<template>
  <NuxtLayout name="page">
    <a class="gotoTop" style="opacity: 0; pointer-events: none" id="gotoTop" href="#top"></a>
    <div id="top"></div>
    <h1 class="page-title">Library</h1>
    <ul class="musicList" id="ul"></ul>
  </NuxtLayout>
</template>

<script>
import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";
import { audioDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";

export default {
  async mounted() {
    var contents = await readTextFile(`savedMusic/_all.json`, {
      dir: BaseDirectory.Audio,
    });
    contents = JSON.parse(contents);
    contents = contents.all

    var searchResults = [];
    const promises = contents.map(async (item) => {
      var music = await readTextFile(`savedMusic/${item}.json`, {
        dir: BaseDirectory.Audio,
      });
      music = JSON.parse(music);
      if (music.imageURL == "") {
        music.imageURL = "/unknown.png";
      }
      if (music.trackName == "") {
        music.trackName = "Unknown Name";
      }
      if (music.artistName == "") {
        music.artistName = "Unknown Artist";
      }
      var music = {
        coverURL: music.imageURL,
        name:
          music.trackName.length > 15
            ? music.trackName.slice(0, 15).at(-1) == " "
              ? music.trackName.slice(0, 14) + "..."
              : music.trackName.slice(0, 15) + "..."
            : music.trackName,
        artist:
          music.artistName.length > 20
            ? music.artistName.slice(0, 20).at(-1) == " "
              ? music.artistName.slice(0, 19) + "..."
              : music.artistName.slice(0, 20) + "..."
            : music.artistName,
        originalName: music.trackName,
        audioFile: music.audioFile
      };
      searchResults.push(music);
    });

    await Promise.all(promises);

    const ul = document.getElementById("ul");

    searchResults.forEach((item) => {
      ul.innerHTML += `
      <li class="musicItem">
        <div class="libaryCover">
          <img class="searchResultCover2" src="${item.coverURL}" />
          <div class="searchResultPlay2" audio="${item.audioFile + ".mp3"
        }"></div>
        </div>
        <div class="searchResultName2">${item.name}</div>
        <div class="searchResultArtist2">${item.artist}</div>
      </li>`;
    });

    var elements = Array.from(
      document.getElementsByClassName("searchResultPlay2")
    );

    elements.forEach(async (item) => {
      item.addEventListener("click", async () => {
        playAudio(item.getAttribute("audio"));

        var music = await readTextFile(`savedMusic/${item.getAttribute("audio")}.json`, {
          dir: BaseDirectory.Audio,
        });
        music = JSON.parse(music);
        if (music.imageURL == "") {
          music.imageURL = "/unknown.png";
        }
        if (music.trackName == "") {
          music.trackName = "Unknown Name";
        }
        if (music.artistName == "") {
          music.artistName = "Unknown Artist";
        }

        const audioPlayed = new CustomEvent("customPlayAudio", {
          bubbles: true,
          cancelable: true,
          detail: {
            image: music.imageURL,
            name: music.trackName,
            artist: music.artistName,
          },
        });
        document.dispatchEvent(audioPlayed);
      });
    });
  },
};

async function playAudio(path) {
  const audioDirPath = await audioDir();
  const filePath = await join(audioDirPath, `savedMusic/${path}`);
  const assetUrl = convertFileSrc(filePath);

  const audio = document.getElementById("media");
  audio.innerHTML = "";
  const source = document.createElement("source");
  source.type = "audio/mp3";
  source.src = assetUrl;
  audio.appendChild(source);
  audio.load();
  audio.play();
}
</script>

<style>
@import "~/css/style.css";

.musicList {
  max-width: 1500px;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(170px, 1fr));
  grid-gap: 20px;
  margin-bottom: 70px;
}

.musicItem {
  aspect-ratio: 1 / 1.3;
  max-width: 200px;
  display: grid;
  grid-template-rows: auto 20px 20px;
  /* First row takes up remaining space, other rows fit content */
  grid-template-columns: 100%;
}

.libaryCover {
  aspect-ratio: 1 / 1;
  align-self: center;
  width: 100%;
  grid-row: 1 / 2;
  grid-column: 1 / 2;
  display: grid;
}

.searchResultCover2 {
  width: 100%;
  border-radius: 30px;
  grid-row: 1 / 2;
  grid-column: 1 / 2;
}

.searchResultPlay2 {
  grid-row: 1 / 2;
  grid-column: 1 / 2;
  z-index: 1;
  display: grid;
  background-color: transparent;
  border: 1px solid rgba(64, 62, 68, 0.24);
  width: 35px;
  height: 35px;
  place-items: center;
  border-radius: 100%;
  align-self: flex-end;
  justify-self: flex-end;
  margin: 15px;
  margin-bottom: 13px;
  background-image: url("/svg/bold/play.svg");
  background-size: 20px;
  background-repeat: no-repeat;
  background-position: 53% 53%;
}

.searchResultPlay2:hover {
  background-color: #1e1d23;
}

.searchResultName2 {
  grid-row: 2 / 3;
  font-weight: bold;
  font-size: 20px;
  white-space: nowrap;
  overflow: hidden;
  align-self: end;
}

.searchResultArtist2 {
  grid-row: 3 / 4;
  color: var(--dtx);
  align-self: flex-start;
}
</style>
