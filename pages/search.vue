<template>
  <div class="main element">
    <p class="element-title">Search</p>
    <div class="search">
      <div class="search-container">
        <IconsSearch />
        <input class="input" spellcheck="false" type="text" v-model="searchTerm" @input="handleInput"
          placeholder="Search" />
      </div>
      <div v-if="searchResults.length > 0" class="results">
        <div class="inline">
          <div class="top-result">
            <p>Top Result</p>
            <div @contextmenu.prevent="showContextMenu($event, searchResults[0])" class="content">
              <img class="cover" :src="searchResults[0].thumbnail" :alt="searchResults[0].title" loading="lazy" />
              <div>
                <div class="title">{{ truncate(searchResults[0].title) }}</div>
                <div class="artist">{{ searchResults[0].uploaderName }}</div>
              </div>
              <div @click="play(searchResults[0])" class="play">
                <svg width="11.083252px" height="14px" viewBox="0 0 11.083252 14" version="1.1"
                  xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg">
                  <path d="M0 0L0 14L11.083252 7L0 0Z" id="Shape" fill="#000000" stroke="none" />
                </svg>
              </div>
              <ContextMenu :x="menuX" :y="menuY" :show="showMenu" :menuItems="menuItems" @close="closeContextMenu" />
            </div>
          </div>

          <div class="songs">
            <p class="songs-title">Songs</p>
            <div class="content">
              <div @contextmenu.prevent="showContextMenu($event, song)"
                v-for="(song, index) in searchResults.slice(1, 6)" :key="getSongId(song)"
                :class="['song', { playing: isCurrentSong(song) }]"
                @mouseover="hoveredSongId = getSongId(song)" @mouseleave="hoveredSongId = ''">
                <div class="inline-songs">
                  <div @click="play(song)" class="cover">
                    <div class="playing-indicator">
                      <div class="bar"></div>
                      <div class="bar"></div>
                      <div class="bar"></div>
                      <div class="bar"></div>
                    </div>
                    <svg v-show="hoveredSongId === getSongId(song)" width="14px" height="14px"
                      viewBox="0 0 14 14" version="1.1" xmlns:xlink="http://www.w3.org/1999/xlink"
                      xmlns="http://www.w3.org/2000/svg">
                      <g id="Group">
                        <path d="M0 0L14 0L14 14L0 14L0 0Z" id="Rectangle" fill="none" fill-rule="evenodd"
                          stroke="none" />
                        <path d="M2 14L2 0L12.5 7L2 14Z" id="Shape" fill="#FFFFFF" stroke="none" />
                      </g>
                    </svg>
                    <img :src="song.thumbnail || '/cover.png'" :alt="song.title" class="img" />
                  </div>
                  <div class="titles">
                    <p class="title">{{ truncateTitle(song.title) }}</p>
                    <p class="artist">{{ truncateArtist(song.uploaderName) }}</p>
                  </div>
                </div>
                <p class="lenght">{{ formatDuration(song.duration) }}</p>
              </div>
              <ContextMenu :x="menuX" :y="menuY" :show="showMenu" :menuItems="menuItems" @close="closeContextMenu" />
            </div>
          </div>
        </div>

        <div class="albums">
          <p>Albums</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { BaseDirectory, writeFile, exists } from '@tauri-apps/plugin-fs';
import axios from 'axios';
import type { Song } from '~/types/types';
import { debounce } from 'lodash-es';

interface Response {
  url: string;
  title: string;
  thumbnail: string;
  uploaderName: string;
  uploaderAvatar: string;
  duration: number;
  durationFormatted: string;
}

const { $player, $settings, $music } = useNuxtApp();

const searchTerm = ref("");
const searchResults = ref<Response[]>([]);
const hoveredSongId = ref("");

const currentSong = computed(() => $player.currentSong);

watch(currentSong, () => { });

async function searchSongs() {
  if (searchTerm.value === "") {
    searchResults.value = [];
    return;
  }

  try {
    const apiURL = await $settings.getApiUrl();

    const response = await fetch(`${apiURL}/search?query=${encodeURIComponent(searchTerm.value)}`);
    const data = await response.json();
    searchResults.value = Object.values(data).map((item: any) => ({
      duration: item.duration,
      durationFormatted: `${Math.floor(item.duration / 60)}:${item.duration % 60 < 10 ? '0' : ''}${item.duration % 60}`,
      thumbnail: item.thumbnailUrl,
      title: item.title,
      uploaderAvatar: '',
      uploaderName: item.artist,
      url: `https://www.youtube.com/watch?v=${item.id}`,
    }));
  } catch (error) {
    console.error("Failed to fetch songs:", error, searchTerm.value);
    searchResults.value = [];
  }
}

async function addToLibrary(song: Response) {
  try {
    const match = song.url.match(/(?:\/watch\?v=)([^&]+)/)! as RegExpMatchArray;

    if (!match || !match[1]) {
      console.error("No valid ID found in the URL.");
      return;
    }

    const videoId = match[1];
    const isLossless = await $settings.getLossless();
    const mp3Exists = await exists(`Vleer/Songs/${videoId}.mp3`, { baseDir: BaseDirectory.Audio });
    const flacExists = await exists(`Vleer/Songs/${videoId}.flac`, { baseDir: BaseDirectory.Audio });

    if ((isLossless && !flacExists) || (!isLossless && !mp3Exists)) {
      const songData: Song = {
        id: videoId,
        title: song.title,
        artist: song.uploaderName,
        duration: song.duration,
        cover: song.thumbnail.replace(/^https?:\/\/[^\/]+/, ''),
        date_added: new Date(),
        album: ''
      };

      try {
        await invoke('download', { id: videoId, quality: isLossless ? 'lossless' : 'compressed' });

        if (!mp3Exists && !flacExists) {
          const response = await axios.get(song.thumbnail.replace("w120-h120", "w500-h500"), { responseType: 'arraybuffer' });
          const data = new Uint8Array(response.data);
          await writeFile(`Vleer/Covers/${videoId}.png`, data, { baseDir: BaseDirectory.Audio });
          await $music.addSong(songData);
        }
      } catch (error) {
        console.error('Error downloading video:', error);
        return;
      }
    }
  } catch (error) {
    console.error("Failed to handle song play:", error);
  }
}

async function play(song: Response) {
  try {
    const match = song.url.match(/(?:\/watch\?v=)([^&]+)/)! as RegExpMatchArray;

    if (!match || !match[1]) {
      console.error("No valid ID found in the URL.");
      return;
    }

    const videoId = match[1];
    const isLossless = await $settings.getLossless();
    const mp3Exists = await exists(`Vleer/Songs/${videoId}.mp3`, { baseDir: BaseDirectory.Audio });
    const flacExists = await exists(`Vleer/Songs/${videoId}.flac`, { baseDir: BaseDirectory.Audio });

    let dbSong = await $music.getSong(videoId);

    if (!dbSong) {
      const songData: Song = {
        album: '',
        artist: song.uploaderName,
        cover: song.thumbnail.replace(/^https?:\/\/[^\/]+/, ''),
        date_added: new Date(),
        duration: song.duration,
        id: videoId,
        title: song.title,
      };

      await $music.addSong(songData);
      dbSong = songData;
    }

    if ((isLossless && !flacExists) || (!isLossless && !mp3Exists)) {
      try {
        await invoke('download', { id: videoId, quality: isLossless ? 'lossless' : 'compressed' });

        if (!mp3Exists && !flacExists) {
          const response = await axios.get(song.thumbnail.replace("w120-h120", "w500-h500"), { responseType: 'arraybuffer' });
          const data = new Uint8Array(response.data);
          await writeFile(`Vleer/Covers/${videoId}.png`, data, { baseDir: BaseDirectory.Audio });
        }
      } catch (error) {
        console.error('Error downloading video:', error);
        return;
      }
    }

    await $player.loadSong(dbSong);
    $player.play();
  } catch (error) {
    console.error("Failed to handle song play:", error);
  }
}

function truncate(text: string) {
  const maxLength = Math.floor(252 / 10);
  if (text.length > maxLength) {
    return text.substring(0, maxLength) + '...';
  }
  return text;
}

function formatDuration(duration: number) {
  const minutes = Math.floor(duration / 60);
  const seconds = duration % 60;
  return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
}

function truncateTitle(text: string) {
  const maxLength = (window.innerWidth - 788) / 16;
  if (text.length > maxLength) {
    return text.substring(0, maxLength) + '...';
  }
  return text;
}

function truncateArtist(text: string) {
  const maxLength = (window.innerWidth - 788) / 20;
  if (text.length > maxLength) {
    return text.substring(0, maxLength) + '...';
  }
  return text;
}

const showMenu = ref(false);
const menuX = ref(0);
const menuY = ref(0);
const menuItems = ref<{ label: string; action: () => void }[]>([]);

function showContextMenu(event: MouseEvent, song: Response) {
  event.preventDefault();
  menuX.value = event.clientX;
  menuY.value = event.clientY;
  showMenu.value = true;

  menuItems.value = [
    {
      label: 'Add to library',
      action: () => {
        addToLibrary(song)
      },
    },
  ];
}

function closeContextMenu() {
  showMenu.value = false;
}

onMounted(() => {
  window.addEventListener('click', closeContextMenu);
});

const handleInput = debounce(() => {
  searchSongs();
}, 300);

function getSongId(song: Response): string {
  const match = song.url.match(/(?:\/watch\?v=)([^&]+)/);
  return match && match[1] ? match[1] : '';
}

function isCurrentSong(song: Response): boolean {
  const songId = getSongId(song);
  return !!currentSong.value && currentSong.value.id === songId;
}
</script>

<style scoped lang="scss">
@import '~/assets/styles/pages/search.scss';
</style>