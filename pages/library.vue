<template>
  <NuxtLayout name="page">
    <h1 class="page-title">Library</h1>
    <ul class="musicList" id="ul"></ul>
  </NuxtLayout>
</template>

<script>
import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";

var searchResults = [];
export default {
  async mounted() {
    var contents = await readTextFile(`savedMusic/_all.json`, {
      dir: BaseDirectory.Audio,
    });
    contents = JSON.parse(contents);

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
          music.trackName.length > 16
            ? music.trackName.slice(0, 16).at(-1) == " "
              ? music.trackName.slice(0, 15) + "..."
              : music.trackName.slice(0, 16) + "..."
            : music.trackName,
        artist:
          music.artistName.length > 20
            ? music.artistName.slice(0, 20).at(-1) == " "
              ? music.artistName.slice(0, 19) + "..."
              : music.artistName.slice(0, 20) + "..."
            : music.artistName,
      };
      searchResults.push(music);
    });

    console.log(searchResults)

    await Promise.all(promises);

    const ul = document.getElementById("ul");

    searchResults.forEach((item) => {
      ul.innerHTML += `<li class="musicItem">
      <img class="searchResultCover2" src="${item.coverURL}" />
      <div class="searchResultPlay2">
        <img src="/svg/bold/play.svg" class="searchResultIMG2" />
        </div>
        <div class="searchResultName2">${item.name}</div>
        <div class="searchResultArtist2">${item.artist}</div>
        </li>`;
    });
  },
};
</script>

<style>
@import "~/css/style.css";

.page-title {
  margin-bottom: 20px;
  margin-top: 10px;
}

.musicList {
  max-width: 1500px;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  grid-gap: 20px;
}

.musicItem {
  max-width: 250px;
  height: 320px;
  display: grid;
  grid-template-rows: auto auto auto; /* First row takes up remaining space, other rows fit content */
  grid-template-columns: 100%;
}

.searchResultCover2 {
  align-self: center;
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
  border: 1px solid rgba(64, 62, 68, 0.24);
  width: 45px;
  height: 45px;
  place-items: center;
  border-radius: 100%;
  align-self: flex-end;
  justify-self: flex-end;
  margin: 15px;
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
