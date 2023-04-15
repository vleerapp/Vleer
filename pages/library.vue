<template>
  <NuxtLayout name="page">
    <h1 class="page-title">Library</h1>
    <ul class="musicList">
      <li class="musicItem" v-for="(result, index) in searchResults" :key="index">
        <img class="searchResultCover" :src="result.coverURL" />
        <div class="searchResultPlay">
          <img src="/svg/bold/play.svg" class="searchResultIMG" />
        </div>
        <div class="searchResultName">{{ result.name }}</div>
        <div class="searchResultArtist">{{ result.artist }}</div>
      </li>
    </ul>
  </NuxtLayout>
</template>

<script setup>
var searchResults = [
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
  { coverURL: "waradu.png", name: "Waradu", artist: "Waradu" },
];
</script>

<style>
@import '~/css/style.css';

.page-title {
  margin-bottom: 20px;
  margin-top: 10px;
}

.musicList {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  grid-gap: 20px;
  margin-bottom: 30px;
}

.musicItem {
  max-width: 250px;
  height: 300px;
  display: grid;
  grid-template-rows: auto auto auto;
  /* First row takes up remaining space, other rows fit content */
  grid-template-columns: 100%;
}

.searchResultCover {
  align-self: center;
  width: 100%;
  border-radius: 10px;
  grid-row: 1 / 2;
  grid-column: 1 / 2;
}

.searchResultPlay {
  grid-row: 1 / 2;
  grid-column: 1 / 2;
  z-index: 1;
  display: grid;
  border: 1px solid rgba(64, 62, 68, 0.24);
  background: #1e1d23;
  width: 45px;
  height: 45px;
  place-items: center;
  border-radius: 100%;
  align-self: flex-end;
  justify-self: flex-end;
  margin: 10px;
}

.searchResultName {
  grid-row: 2 / 3;
  font-weight: bold;
  font-size: 20px;
  align-self: end;
}

.searchResultArtist {
  grid-row: 3 / 4;
  color: var(--dtx);
  align-self: flex-start;
}
</style>