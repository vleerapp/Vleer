<template>
  <div class="main element">
    <p class="element-title">Search</p>
    <div class="search">
      <div class="search-container">
        <IconsSearch />
        <input class="input" spellcheck="false" type="text" v-model="searchTerm" @input="handleInput"
          placeholder="Search" />
      </div>
      <ul>
        <li v-for="(song, index) in searchResults" :class="{ 'first-result': index === 0 }"
          @click="handleSongClick(song)">
          <img :src="song.thumbnail" :alt="song.title" loading="lazy" />
          <div>{{ song.title }}</div>
          <div>{{ song.uploaderName }}</div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core';
import { BaseDirectory, writeFile } from '@tauri-apps/plugin-fs';
import axios from 'axios';
import type { MusicSearchResponseItem, MusicSearchResponse, Song } from '~/types/types';
const { $music, $settings } = useNuxtApp();

const searchTerm = ref("");
const searchResults = ref<MusicSearchResponseItem[]>([]);
const isLoading = ref(false);
let searchTimeout: ReturnType<typeof setTimeout>;

async function searchSongs() {
  isLoading.value = true;

  if (searchTerm.value === "") {
    searchResults.value = [];
    isLoading.value = false;
    return;
  }

  let apiURL = $settings.getApiURL()

  if (!apiURL) {
    apiURL = "https://pipedapi.r4fo.com";
  }

  try {
    const response = await fetch(`${apiURL}/search?q=${encodeURIComponent(searchTerm.value)}&filter=music_songs`);
    const data = await response.json();
    searchResults.value = data.items
      .filter(item => item.type !== 'channel')
      .map(item => ({
        url: item.url,
        title: item.title,
        thumbnail: item.thumbnail,
        uploaderName: item.uploaderName,
        uploaderAvatar: item.uploaderAvatar,
        duration: item.duration,
        durationFormatted: `${Math.floor(item.duration / 60)}:${item.duration % 60 < 10 ? '0' : ''}${item.duration % 60}`
      }));
  } catch (error) {
    console.error("Failed to fetch songs:", error, apiURL, searchTerm.value);
    searchResults.value = [];
  }

  isLoading.value = false;
}

function handleInput() {
  clearTimeout(searchTimeout);
  searchTimeout = setTimeout(() => {
    if (searchTerm.value.trim()) {
      searchSongs();
    }
  }, 500);
}

async function handleSongClick(song: MusicSearchResponseItem) {
  try {
    const match = song.url.match(/(?:\/watch\?v=)([^&]+)/)! as RegExpMatchArray;

    if (!match || !match[1]) {
      console.error("No valid ID found in the URL.");
      return;
    }

    const videoId = match[1];

    const songsConfig = $music.getSongs();

    const songExists = Object.values(songsConfig.songs).some(song => song.id === videoId);

    if (songExists) {
      console.log("Song already exists.");
      return;
    }

    var songData: Song = {
      id: videoId,
      title: song.title,
      artist: song.uploaderName,
      length: song.duration,
      cover: song.thumbnail,
      date_added: formatDate(new Date())
    }

    try {
      await invoke('download', { url: "https://youtube.com" + song.url, name: videoId + ".webm" });

      const response = await axios.get(song.thumbnail.replace("w120-h120", "w500-h500"), { responseType: 'arraybuffer' });
      const data = new Uint8Array(response.data);

      await writeFile(`Vleer/Covers/${videoId}.png`, data, { baseDir: BaseDirectory.Audio });

      await $music.addSongData(songData)

      await $music.setSong(videoId)
      $music.play();
    } catch (error) {
      console.error('Error downloading video as MP3:', error);
    }
  } catch (error) {
    console.error("Failed to handle song click:", error);
  }
}

const formatDate = (date: Date) => {
  let year = date.getFullYear();
  let month = (date.getMonth() + 1).toString().padStart(2, '0');
  let day = date.getDate().toString().padStart(2, '0');
  let hours = date.getHours().toString().padStart(2, '0');
  let minutes = date.getMinutes().toString().padStart(2, '0');
  let seconds = date.getSeconds().toString().padStart(2, '0');
  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}
</script>

<style scoped lang="scss">
@import '~/assets/styles/pages/search.scss';
</style>
