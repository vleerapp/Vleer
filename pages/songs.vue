<template>
  <div class="main element">
    <p class="element-title">Songs</p>
    <div class="songs">
      <div class="search-container">
        <IconsSearch />
        <input class="input" placeholder="Search" spellcheck="false" v-model="searchQuery" />
      </div>
      <div class="songs-info">
        <div class="cover">#</div>
        <div class="title">Title</div>
        <div class="date">Date added</div>
        <div class="lenght">
          <img src="/Length.svg" alt="" />
        </div>
      </div>
      <div class="items">
        <div v-for="(song, index) in filteredSongs" :key="song.id" @click="play(song.id, index)"
          :class="['song', { playing: currentSong.id === song.id }]" @mouseover="hoveredSongId = song.id"
          @mouseleave="hoveredSongId = ''">
          <div class="cover">
            <svg v-show="hoveredSongId === song.id" width="14px" height="14px" viewBox="0 0 14 14" version="1.1"
              xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg">
              <g id="Group">
                <path d="M0 0L14 0L14 14L0 14L0 0Z" id="Rectangle" fill="none" fill-rule="evenodd" stroke="none" />
                <path d="M2 14L2 0L12.5 7L2 14Z" id="Shape" fill="#FFFFFF" stroke="none" />
              </g>
            </svg>
            <img :src="song.coverURL || '/cover.png'" :alt="song.title" class="img" />
          </div>
          <div class="titles">
            <p class="title">{{ truncate(song.title) }}</p>
            <p class="artist">{{ truncate(song.artist) }}</p>
          </div>
          <p class="date">{{ formatDate(song.date_added) }}</p>
          <p class="lenght">{{ formatDuration(song.length) }}</p>
        </div>
        <NuxtLink to="/search" class="add">
          <div class="cover">
            <svg width="36px" height="36px" viewBox="0 0 36 36">
              <path d="M0 0L36 0L36 36L0 36L0 0Z" id="Rectangle" fill="none" fill-rule="evenodd" stroke="none" />
              <path d="M17.4 12L18.6 12L18.6 24L17.4 24L17.4 12Z" id="Rectangle" fill="currentColor" fill-rule="evenodd"
                stroke="none" />
              <path d="M24 17.4L24 18.6L12 18.6L12 17.4L24 17.4Z" id="Rectangle" fill="currentColor" fill-rule="evenodd"
                stroke="none" />
            </svg>
          </div>
          <div class="title">Add Songs</div>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { type Song } from "~/types/types";
import { ref, onMounted, watch } from "vue";

const { $music } = useNuxtApp();

const searchQuery = ref("");
const songs = $music.getSongs();
const hoveredSongId = ref("");

const filteredSongs = computed<Song[]>(() => {
  if (!searchQuery.value.trim()) {
    return songs.sort((a, b) => new Date(b.date_added).getTime() - new Date(a.date_added).getTime());
  }
  return songs.filter(song =>
    song.title.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    song.artist.toLowerCase().includes(searchQuery.value.toLowerCase())
  ).sort((a, b) => new Date(b.date_added).getTime() - new Date(a.date_added).getTime());
});

async function play(id: string, index: number) {
  const queueIds = filteredSongs.value.slice(index).map(song => song.id);
  await $music.setQueue(queueIds);
}

function truncate(text: string, length: number = 45) {
  return text.length > length ? text.substring(0, length - 3).trim() + "..." : text;
}

function formatDate(dateString: string) {
  const date = new Date(dateString);
  const day = date.getDate();
  const month = date.getMonth() + 1;
  const year = date.getFullYear();
  return `${day < 10 ? '0' : ''}${day}.${month < 10 ? '0' : ''}${month}.${year}`;
}

function formatDuration(duration: number) {
  const minutes = Math.floor(duration / 60);
  const seconds = duration % 60;
  return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
}

const currentSong = computed(() => {
  return $music.getCurrentSong() || { id: 0 };
});

watch(currentSong, () => {});
</script>

<style scoped lang="scss">
@import "~/assets/styles/pages/songs.scss";
</style>