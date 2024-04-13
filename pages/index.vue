<template>
  <div class="main element">
    <p class="element-title">Home</p>
    <div class="index">
      <div class="eq-controls">
        <div v-for="(freq, index) in frequencies" :key="freq" class="eq-control">
          <input
            type="range"
            min="-12"
            max="12"
            step="0.1"
            v-model.number="eqGains[index]"
            @input="updateEqGain(index, eqGains[index])"
          />
          <label>{{ freq }} Hz</label>
          <span>{{ eqGains[index].toFixed(1) }}</span>
        </div>
      </div>
      <div v-for="song in songs" :key="song.id" class="song-item">
        <img :src="song.coverURL" :alt="song.title" class="song-cover">
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
import type { Song } from '~/types/types';

const { $music } = useNuxtApp()

await $music.init()

const frequencies = [32, 64, 125, 250, 500, 1000, 2000, 4000, 8000, 16000];
const eqGains = ref(new Array(frequencies.length).fill(0));

function updateEqGain(filterIndex, gain) {
  $music.setEqGain(filterIndex, gain);
  const eqSettingsMap = new Map();
  frequencies.forEach((freq, index) => {
    eqSettingsMap.set(freq.toString(), eqGains.value[index]);
  });
}

const songs = ref<Song[]>([]);

onMounted(async () => {
  const loadedSongs = await $music.getSongs();
  const songArray = Object.values(loadedSongs.songs);
  await Promise.all(songArray.map(async song => {
    song.coverURL = await $music.getCoverURLFromID(song.id);
  }));
  songs.value = songArray;
});

async function play(id: string) {
  await $music.setSong(id)
  $music.play()
}

function formatDuration(duration: number) {
  const minutes = Math.floor(duration / 60);
  const seconds = duration % 60;
  return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
}

function formatDate(dateString: string) {
  const date = new Date(dateString);
  const day = date.getDate();
  const month = date.getMonth() + 1;
  const year = date.getFullYear();
  return `${day < 10 ? '0' : ''}${day}.${month < 10 ? '0' : ''}${month}.${year}`;
}
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