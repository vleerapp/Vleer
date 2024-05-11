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
        <div class="cards" ref="cards">
          <div v-for="song in sortedRecentlyPlayed" :key="song.id" @click="play(song.id)" class="song">
            <img :src="song.coverURL" :alt="song.title" class="cover" />
            <div class="info">
              <p class="title" :title="song.title">{{ song.title }}</p>
              <p class="artist" :title="song.artist">{{ song.artist }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type Song } from "~/types/types";
import { computed, ref, onMounted, onUnmounted, watch } from 'vue';
import { useNuxtApp } from '#imports';

const { $music } = useNuxtApp();

const songs = ref<Song[]>([]);
const cards = ref(null);
const maxCards = ref(5);
const cardsWidth = ref(0);
const cardMinWidth = 140; 
const cardMaxWidth = 180; 
const cardGap = 16;

const updateWidth = () => {
  cardsWidth.value = cards.value?.clientWidth || 0;
  updateMaxCards();
};

const updateMaxCards = () => {
  const maxPossible = Math.floor(cardsWidth.value / (cardMinWidth + cardGap));
  const minPossible = Math.floor(cardsWidth.value / (cardMaxWidth + cardGap));
  maxCards.value = maxPossible;
};

watch(cards, (newVal, oldVal) => {
  if (newVal !== oldVal) {
    updateWidth();
  }
});

onMounted(async () => {
  const loadedSongs = await $music.getSongs();
  console.log(loadedSongs);
  const songArray = Object.values(loadedSongs.songs);
  await Promise.all(
    songArray.map(async (song) => {
      song.coverURL = await $music.getCoverURLFromID(song.id);
    })
  );
  songs.value = songArray;
  updateWidth();
  window.addEventListener('resize', updateWidth);
});

onUnmounted(() => {
  window.removeEventListener('resize', updateWidth);
});

const sortedRecentlyPlayed = computed(() => {
  return [...songs.value]
    .filter(song => song.lastPlayed)
    .sort((a, b) => new Date(b.lastPlayed).getTime() - new Date(a.lastPlayed).getTime())
    .slice(0, maxCards.value);
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