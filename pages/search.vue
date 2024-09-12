<template>
  <div class="main element">
    <p class="element-title">Search</p>
    <div class="search">
      <div v-if="searchResults.length > 0" class="results">
        <div class="inline">
          <div class="top-result">
            <p>Top Result</p>
            <div @contextmenu.prevent="showContextMenu($event, searchResults[0])" class="content">
              <img class="cover" :src="searchResults[0].cover" :alt="searchResults[0].title" loading="lazy" />
              <div>
                <div class="title">{{ searchResults[0].title }}</div>
                <div class="artist">{{ searchResults[0].artist }}</div>
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
                v-for="(song, index) in searchResults.slice(1, 6)" :key="song.id"
                :class="['song', { playing: isCurrentSong(song) }]" @mouseover="hoveredSongId = song.id"
                @mouseleave="hoveredSongId = ''">
                <div class="inline-songs">
                  <div @click="play(song)" class="cover">
                    <div class="playing-indicator">
                      <div class="bar"></div>
                      <div class="bar"></div>
                      <div class="bar"></div>
                      <div class="bar"></div>
                    </div>
                    <svg v-show="hoveredSongId === song.id" width="14px" height="14px" viewBox="0 0 14 14" version="1.1"
                      xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg">
                      <g id="Group">
                        <path d="M0 0L14 0L14 14L0 14L0 0Z" id="Rectangle" fill="none" fill-rule="evenodd"
                          stroke="none" />
                        <path d="M2 14L2 0L12.5 7L2 14Z" id="Shape" fill="#FFFFFF" stroke="none" />
                      </g>
                    </svg>
                    <img :src="song.cover || '/cover.png'" :alt="song.title" class="img" />
                  </div>
                  <div class="titles">
                    <p class="title">{{ song.title }}</p>
                    <p class="artist">{{ song.artist }}</p>
                  </div>
                </div>
                <p class="length">{{ formatDuration(song.duration) }}</p>
              </div>
              <ContextMenu :x="menuX" :y="menuY" :show="showMenu" :menuItems="menuItems" @close="closeContextMenu" />
            </div>
          </div>
        </div>

        <div class="albums">
          <div class="section-header">
            <p>Albums</p>
            <div class="scroll-buttons">
              <button class="scroll-button" @click="scroll('albums', 'left')" :disabled="albumsScrollLeft === 0">&lt;</button>
              <button class="scroll-button" @click="scroll('albums', 'right')" :disabled="albumsScrollLeft >= albumsMaxScroll">&gt;</button>
            </div>
          </div>
          <div class="album-grid" ref="albumsGrid">
            <div v-for="album in albums" :key="album.id" class="album-item">
              <img :src="album.cover" :alt="album.name" class="album-cover" />
              <p class="album-title">{{ album.name }}</p>
              <p class="album-artist">{{ album.author }}</p>
            </div>
          </div>
        </div>

        <div class="playlists">
          <div class="section-header">
            <p>Playlists</p>
            <div class="scroll-buttons">
              <button class="scroll-button" @click="scroll('playlists', 'left')" :disabled="playlistsScrollLeft === 0">&lt;</button>
              <button class="scroll-button" @click="scroll('playlists', 'right')" :disabled="playlistsScrollLeft >= playlistsMaxScroll">&gt;</button>
            </div>
          </div>
          <div class="playlist-grid" ref="playlistsGrid">
            <div v-for="playlist in playlists" :key="playlist.id" class="playlist-item">
              <img :src="playlist.cover" :alt="playlist.name" class="playlist-cover" />
              <p class="playlist-title">{{ playlist.name }}</p>
              <p class="playlist-owner">{{ playlist.author }}</p>
            </div>
          </div>
        </div>
      </div>
      <div v-else-if="searchTerm" class="no-results">
        No results found for "{{ searchTerm }}"
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, watch, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { BaseDirectory, writeFile, exists } from '@tauri-apps/plugin-fs';
import axios from 'axios';
import type { Song } from '~/types/types';

export interface Response {
  id: string;
  title: string;
  artist: string;
  album: string;
  cover: string;
  duration: number;
}

export interface Album {
  id: string;
  name: string;
  author: string;
  cover: string;
  songs: Song[];
}

export interface Playlist {
  id: string;
  name: string;
  author: string;
  cover: string;
  songs: Song[];
}

const route = useRoute();
const searchTerm = ref('');
const searchResults = ref<Response[]>([]);
const hoveredSongId = ref('');
const albums = ref<Album[]>([]);
const playlists = ref<Playlist[]>([]);

const { $player, $settings, $music } = useNuxtApp();

const currentSong = computed(() => $player.currentSong);

watch(currentSong, () => { });

watch(() => route.query.q, (newQuery) => {
  searchTerm.value = newQuery as string || '';
  if (searchTerm.value) {
    searchSongs();
  } else {
    searchResults.value = [];
    albums.value = [];
    playlists.value = [];
  }
});

onMounted(() => {
  searchTerm.value = route.query.q as string || '';
  if (searchTerm.value) {
    searchSongs();
  }
});

let currentSearchId = 0;

async function searchSongs() {
  if (searchTerm.value === "") {
    searchResults.value = [];
    albums.value = [];
    playlists.value = [];
    return;
  }

  const searchId = ++currentSearchId;

  try {
    const apiURL = await $settings.getApiUrl();

    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 10000);

    const response = await fetch(`${apiURL}/search?query=${encodeURIComponent(searchTerm.value)}&mode=minimal`, {
      signal: controller.signal
    });

    clearTimeout(timeoutId);

    if (searchId !== currentSearchId) {
      return;
    }

    const data = await response.json();
    
    searchResults.value = Object.values(data.songs).map((song: any) => ({
      id: song.id,
      title: song.title,
      artist: song.artist,
      album: song.album,
      cover: song.cover,
      duration: song.duration,
    }));

    albums.value = Object.values(data.albums).map((album: any) => ({
      id: album.id,
      name: album.name,
      author: album.author,
      cover: album.cover,
      songs: album.songs,
    }));

    playlists.value = Object.values(data.playlists).map((playlist: any) => ({
      id: playlist.id,
      name: playlist.name,
      author: playlist.author,
      cover: playlist.cover,
      songs: playlist.songs,
    }));

    console.log(searchResults.value, albums.value, playlists.value);
  } catch (error: any) {
    if (error.name === 'AbortError') {
      console.log("Search request timed out");
    } else {
      console.error("Failed to fetch search results:", error, searchTerm.value);
    }
  }
}

async function addToLibrary(song: Response) {
  try {
    const isLossless = await $settings.getLossless();
    const mp3Exists = await exists(`Vleer/Songs/${song.id}.mp3`, { baseDir: BaseDirectory.Audio });
    const flacExists = await exists(`Vleer/Songs/${song.id}.flac`, { baseDir: BaseDirectory.Audio });

    if ((isLossless && !flacExists) || (!isLossless && !mp3Exists)) {
      const songData: Song = {
        id: song.id,
        title: song.title,
        artist: song.artist,
        duration: song.duration,
        cover: `/thumbnail?id=${song.id}`,
        date_added: new Date(),
        album: ''
      };

      try {
        await invoke('download', { id: song.id, quality: isLossless ? 'lossless' : 'compressed', url: await $settings.getApiUrl() });

        if (!mp3Exists && !flacExists) {
          const response = await axios.get(song.cover.replace("w120-h120", "w500-h500"), { responseType: 'arraybuffer' });
          const data = new Uint8Array(response.data);
          await writeFile(`Vleer/Covers/${song.id}.png`, data, { baseDir: BaseDirectory.Audio });
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
    const isLossless = await $settings.getLossless();
    const mp3Exists = await exists(`Vleer/Songs/${song.id}.mp3`, { baseDir: BaseDirectory.Audio });
    const flacExists = await exists(`Vleer/Songs/${song.id}.flac`, { baseDir: BaseDirectory.Audio });

    let dbSong = await $music.getSong(song.id);

    if (!dbSong) {
      const songData: Song = {
        album: '',
        artist: song.artist,
        cover: `/thumbnail?id=${song.id}`,
        date_added: new Date(),
        duration: song.duration,
        id: song.id,
        title: song.title,
      };

      await $music.addSong(songData);
      dbSong = songData;
    }

    if ((isLossless && !flacExists) || (!isLossless && !mp3Exists)) {
      try {
        await invoke('download', { id: song.id, quality: isLossless ? 'lossless' : 'compressed', url: await $settings.getApiUrl() });

        if (!mp3Exists && !flacExists) {
          const response = await axios.get(song.cover.replace("w120-h120", "w500-h500"), { responseType: 'arraybuffer' });
          const data = new Uint8Array(response.data);
          await writeFile(`Vleer/Covers/${song.id}.png`, data, { baseDir: BaseDirectory.Audio });
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

function formatDuration(duration: number) {
  const minutes = Math.floor(duration / 60);
  const seconds = duration % 60;
  return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
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

function isCurrentSong(song: Response): boolean {
  return !!currentSong.value && currentSong.value.value?.id === song.id;
}

const albumsGrid = ref<HTMLElement | null>(null);
const playlistsGrid = ref<HTMLElement | null>(null);
const albumsScrollLeft = ref(0);
const playlistsScrollLeft = ref(0);
const albumsMaxScroll = ref(0);
const playlistsMaxScroll = ref(0);

function updateMaxScroll() {
  if (albumsGrid.value) {
    albumsMaxScroll.value = albumsGrid.value.scrollWidth - albumsGrid.value.clientWidth;
  }
  if (playlistsGrid.value) {
    playlistsMaxScroll.value = playlistsGrid.value.scrollWidth - playlistsGrid.value.clientWidth;
  }
}

function scroll(type: 'albums' | 'playlists', direction: 'left' | 'right') {
  const grid = type === 'albums' ? albumsGrid.value : playlistsGrid.value;
  if (!grid) return;

  const scrollAmount = 200;
  const newScrollLeft = direction === 'left'
    ? Math.max(0, grid.scrollLeft - scrollAmount)
    : Math.min(grid.scrollLeft + scrollAmount, grid.scrollWidth - grid.clientWidth);

  grid.scrollTo({
    left: newScrollLeft,
    behavior: 'smooth'
  });

  if (type === 'albums') {
    albumsScrollLeft.value = newScrollLeft;
  } else {
    playlistsScrollLeft.value = newScrollLeft;
  }
}

watch([albums, playlists], () => {
  setTimeout(updateMaxScroll, 0);
});

onMounted(() => {
  window.addEventListener('resize', updateMaxScroll);
  updateMaxScroll();
});
</script>

<style scoped lang="scss">
@import '~/assets/styles/pages/search.scss';
</style>