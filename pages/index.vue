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
        <div class="cards" ref="playlist_cards">
          <template v-for="n in 6">
            <div v-if="sortedPlaylists.value && sortedPlaylists.value.length > n" :key="sortedPlaylists.value[n].id"
              class="playlist">
              <NuxtLink :to="'/' + sortedPlaylists.value[n].id">
                <img :src="sortedPlaylists.value[n].coverURL || '/cover.png'" height="64px" alt="playlist cover"
                  class="cover">
                <p class="name">{{ truncate(sortedPlaylists.value[n].name) }}</p>
              </NuxtLink>
              <button class="play">
                <svg width="10.5px" height="14px" viewBox="0 0 10.5 14" version="1.1"
                  xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg">
                  <g id="Group">
                    <path d="M0 14L0 0L10.5 7L0 14Z" id="Shape" fill="#000000" stroke="none" />
                  </g>
                </svg>
              </button>
            </div>
            <div v-else :key="n" class="playlist placeholder">
              <img src="/cover.png" height="64px" alt="loading" class="cover">
              <p class="name">Loading...</p>
              <button class="play">
                <svg width="10.5px" height="14px" viewBox="0 0 10.5 14" version="1.1"
                  xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg">
                  <g id="Group">
                    <path d="M0 14L0 0L10.5 7L0 14Z" id="Shape" fill="#000000" stroke="none" />
                  </g>
                </svg>
              </button>
            </div>
          </template>
        </div>
      </div>

      <div class="recently-played">
        <div class="title">Recently played</div>
        <div class="cards" ref="song_cards">
          <div v-for="song in sortedRecentlyPlayed" :key="song.id" @click="play(song.id)" class="song">
            <img :src="song.coverURL || '/cover.png'" :alt="song.title" class="cover" />
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

<script setup>
import { useMusicStore } from "~/stores/music";

const { $music } = useNuxtApp();
const musicStore = useMusicStore();

let songs = $music.getSongs();
const playlists = $music.getPlaylists();

const playlist_cards = ref(null)
const song_cards = ref(null)
const maxCards = ref(5)
const cardsWidth = ref(0)
const cardMinWidth = 180
const cardMaxWidth = 238
const cardGap = 16

function updateWidthSongs() {
  if (song_cards.value && song_cards.value.clientWidth) {
    const clientWidth = song_cards.value.clientWidth;
    cardsWidth.value = clientWidth;
    updateMaxCardsDirect(clientWidth);
  }
}

function updateMaxCardsDirect(clientWidth) {
  if (clientWidth > 0) {
    const maxPossible = Math.floor(clientWidth / (cardMinWidth + cardGap));
    maxCards.value = maxPossible;
  }
}

function updateWidthPlaylists() {
  if (playlist_cards.value) {
    const width = playlist_cards.value.clientWidth;
    const final = Math.round((width - 32) / 3);
    playlist_cards.value.style.gridTemplateColumns = `repeat(3, ${final}px)`;
  }
}

function play(id) {
  if ($music && $music.setSong && $music.play) {
    $music.setSong(id).then(() => {
      $music.play();
    });
  }
}

function truncate(text, length = 24) {
  return text.length > length ? text.substring(0, length - 3) + '...' : text;
}

const sortedPlaylists = computed(() => {
  return playlists.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
});

const sortedRecentlyPlayed = computed(() => {
  return songs.filter(song => song.last_played)
    .sort((a, b) => new Date(b.last_played).getTime() - new Date(a.last_played).getTime())
    .slice(0, maxCards.value);
});

watch(() => musicStore.songsConfig.songs, (newSongs) => {
  songs = newSongs;
}, { deep: true });

onMounted(async () => {
  updateWidthSongs();
  updateWidthPlaylists();

  window.addEventListener('resize', updateWidthSongs);
  window.addEventListener('resize', updateWidthPlaylists);
})
</script>

<style scoped lang="scss">
@import "~/assets/styles/pages/index.scss";
</style>
