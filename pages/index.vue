<template>
  <div class="main element">
    <p class="element-title">Home</p>
    <div class="index">
      <pre class="ascii">
                __                        
 _      _____  / /________  ____ ___  ___ 
| | /| / / _ \/ / ___/ __ \/ __ `__ \/ _ \
| |/ |/ /  __/ / /__/ /_/ / / / / / /  __/
|__/|__/\___/_/\___/\____/_/ /_/ /_/\___/ 
      </pre>

      <div class="playlists">
        <div class="title">Playlists</div>
        <div class="cards"></div>
      </div>

      <div class="recently-played">
        <div class="title">Recently played</div>
        <div class="cards">
          <div v-for="song in sortedRecentlyPlayed" :key="song.id" @click="play(song.id)" class="song">
            <nuxt-img :src="song.coverURL" :alt="song.title" class="cover" />
            <div class="info">
              <p class="title">{{ truncate(song.title) }}</p>
              <p class="artist">{{ truncate(song.artist) }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { type Song } from "~/types/types";
import { computed, ref, onMounted } from 'vue';

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

const sortedRecentlyPlayed = computed(() => {
  return [...songs.value]
    .filter(song => song.lastPlayed)
    .sort((a, b) => new Date(b.lastPlayed).getTime() - new Date(a.lastPlayed).getTime())
    .slice(0, 7);
});

async function play(id: string) {
  await $music.setSong(id);
  $music.play();
}

function truncate(text: string, length: number = 24) {
  return text.length > length ? text.substring(0, length - 3) + '...' : text;
}
</script>

<style scoped lang="scss">
@import "~/assets/styles/pages/index.scss";
</style>