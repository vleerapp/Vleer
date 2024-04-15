<template>
  <div class="main element">
    <p class="element-title">Home</p>
    <div class="index">
      <pre class="ascii-art">
                __                        
 _      _____  / /________  ____ ___  ___ 
| | /| / / _ \/ / ___/ __ \/ __ `__ \/ _ \
| |/ |/ /  __/ / /__/ /_/ / / / / / /  __/
|__/|__/\___/_/\___/\____/_/ /_/ /_/\___/ 
      </pre>
      <div v-for="song in songs" :key="song.id" class="song-item">
        <img :src="song.coverURL" :alt="song.title" class="song-cover" />
        <p v-if="!song.id" class="error">Song ID is missing</p>
        <div class="song-info">
          <h2>{{ song.title }}</h2>
          <p>{{ song.artist }}</p>
          <p>{{ formatDuration(song.length) }}</p>
          <p>{{ formatDate(song.date_added) }}</p>
        </div>
        <button @click="play(song.id)">Play</button>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { type Song } from "~/types/types";

const { $music } = useNuxtApp();

const songs = ref<Song[]>([]);

onMounted(async () => {
  const loadedSongs = await $music.getSongs();
  const songArray = Object.values(loadedSongs.songs);
  await Promise.all(
    songArray.map(async (song) => {
      song.coverURL = await $music.getCoverURLFromID(song.id);
    })
  );
  songs.value = songArray;
});

async function play(id: string) {
  await $music.setSong(id);
  $music.play();
}

function formatDuration(duration: number) {
  const minutes = Math.floor(duration / 60);
  const seconds = duration % 60;
  return `${minutes}:${seconds < 10 ? "0" : ""}${seconds}`;
}

function formatDate(dateString: string) {
  const date = new Date(dateString);
  const day = date.getDate();
  const month = date.getMonth() + 1;
  const year = date.getFullYear();
  return `${day < 10 ? "0" : ""}${day}.${
    month < 10 ? "0" : ""
  }${month}.${year}`;
}
</script>

<style scoped lang="scss">
@import "~/assets/styles/pages/index.scss";

.song-item {
  display: flex;
  align-items: center;
  margin-bottom: 20px;
}

.song-cover {
  width: 100px;
  height: 100px;
  object-fit: cover;
  margin-right: 20px;
}

.song-info h2 {
  margin: 0;
  font-size: 20px;
}

.song-info p {
  margin: 5px 0;
  font-size: 16px;
}
</style>
