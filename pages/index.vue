<template>
  <div>
    <h1>Songs List</h1>
    <ul>
      <li v-for="song in songs" :key="song.id" class="song-item">
        <img :src="song.cover" :alt="song.title" class="song-cover">
        <div class="song-info">
          <h2>{{ song.title }}</h2>
          <p>Artist: {{ song.artist }}</p>
          <p>Length: {{ song.length }} seconds</p>
          <p>Date Added: {{ song.dateAdded }}</p>
        </div>
      </li>
    </ul>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue';
import { readSongs } from '~/lib/Config.ts';

const songs = ref([]);

onMounted(async () => {
  const songsConfig = await readSongs();
  songs.value = Object.values(songsConfig.songs);
});
</script>

<style lang="scss">
@import '~/assets/styles/pages/index.scss';

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