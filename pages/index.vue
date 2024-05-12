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
import { computed, ref, onMounted, watch } from 'vue';
import { useNuxtApp } from '#imports';

const { $music } = useNuxtApp();

const cards = ref(null);
const maxCards = ref(5);
const cardsWidth = ref(0);
const cardMinWidth = 180;
const cardMaxWidth = 238;
const cardGap = 16;

const updateWidth = () => {
  if (cards.value) {
    cardsWidth.value = cards.value.clientWidth;
    updateMaxCards();
  }
};

const updateMaxCards = () => {
  if (cardsWidth.value > 0) {
    const maxPossible = Math.floor(cardsWidth.value / (cardMinWidth + cardGap));
    maxCards.value = maxPossible;
  }
};

watch(cards, (newVal, oldVal) => {
  if (newVal !== oldVal) {
    updateWidth();
  }
});

const computedSongs = computed(() => Object.values($music.getSongs().songs));
const songs = ref([])
songs.value = computedSongs.value;

onMounted(async () => {
  songs.value.forEach(async (song) => {
    // song.coverURL = await $music.getCoverURLFromID(song.id);
  });

  updateWidth();
  window.addEventListener('resize', updateWidth);
});

onUnmounted(() => {
  window.removeEventListener('resize', updateWidth);
});

const sortedRecentlyPlayed = computed(() => {
  const filteredSongs = songs.value.filter(song => song.last_played);
  const sortedSongs = filteredSongs.sort((a, b) => new Date(b.last_played).getTime() - new Date(a.last_played).getTime());
  return sortedSongs.slice(0, maxCards.value);
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