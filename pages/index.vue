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
        <div class="cards">
          <template v-for="n in 6" :key="n">
            <div v-if="sortedPlaylists && sortedPlaylists.length > n - 1" class="playlist">
              <NuxtLink :to="'/' + sortedPlaylists[n - 1].id">
                <img :src="sortedPlaylists[n - 1].songs[0].cover || '/cover.png'" alt="playlist cover" class="cover">
                <p class="name">{{ truncate(sortedPlaylists[n - 1].name) }}</p>
              </NuxtLink>
              <button class="play" @click="playPlaylist(sortedPlaylists[n - 1].id)">
                <svg width="10.5px" height="14px" viewBox="0 0 10.5 14" version="1.1"
                  xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg">
                  <g id="Group">
                    <path d="M0 14L0 0L10.5 7L0 14Z" id="Shape" fill="#000000" stroke="none" />
                  </g>
                </svg>
              </button>
            </div>
            <div v-else class="playlist placeholder">
              <img src="/cover.png" alt="loading" class="cover">
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
        <div class="cards">
          <div v-for="song in sortedRecentlyPlayed" :key="song.id" @click="playSong(song.id)" class="song">
            <img :src="song.cover || '/cover.png'" :alt="song.title" class="cover" />
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

<script lang="ts" setup>
import { onMounted, ref, computed } from 'vue';
import type { Playlist, Song } from '~/types/types';

const { $music, $settings, $player } = useNuxtApp();

const playlists = ref<Playlist[]>([]);
const sortedRecentlyPlayed = ref<Song[]>([]);

const sortedPlaylists = computed(() => {
  return playlists.value.sort((a, b) => b.date_created.getTime() - a.date_created.getTime());
});

async function playSong(id: string) {
  const song = await $music.getSong(id);
  if (song) {
    await $settings.setCurrentSong(song);
    $player.loadSong(song);
    $player.play();
  }
}

async function playPlaylist(id: string) {
  const playlist = await $music.getPlaylist(id);
  if (playlist && playlist.songs.length > 0) {
    await $settings.setQueue(playlist.songs);
    await playSong(playlist.songs[0].id);
  }
}

function truncate(text: string, length = 24) {
  return text.length > length ? text.substring(0, length - 3) + '...' : text;
}

onMounted(async () => {
  playlists.value = await $music.getPlaylists();

  const history = await $music.getHistory();
  sortedRecentlyPlayed.value = history
    .sort((a, b) => b.date_played.getTime() - a.date_played.getTime())
    .slice(0, 5)
    .map(item => item.song);
});
</script>

<style scoped lang="scss">
@use "~/assets/styles/pages/index.scss";
</style>
