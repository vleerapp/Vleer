<template>
  <div class="main element">
    <p class="element-title">Playlist</p>
    <div class="playlist">
      <div class="playlist-top">
        <img :src="playlistCover" class="cover" @click="selectNewCover">
        <div class="text">
          <input spellcheck="false" autocapitalize="false" v-model="playlistName" class="name"
            @input="updatePlaylistName" />
          <p class="info">{{ songsCountAndDuration }}</p>
        </div>
        <div class="controls">
          <button class="play"></button>
          <button class="shuffle"></button>
          <button class="dots"></button>
        </div>
        <div class="search-container">
          <IconsSearch />
          <input class="input" placeholder="Search" spellcheck="false" v-model="searchQuery" />
        </div>
      </div>
      <div class="songs">
        <div class="songs-info">
          <div class="cover">#</div>
          <div class="title">Title</div>
          <div class="date">Date added</div>
          <div class="length">
            <img src="/Length.svg" alt="" />
          </div>
        </div>
        <div class="items">
          <div v-for="(song, index) in filteredSongs" :key="song.id" @click="playSong(song.id)" class="song">
            <img :src="song.cover || '/cover.png'" :alt="song.title" class="cover" />
            <div class="titles">
              <p class="title">{{ truncate(song.title) }}</p>
              <p class="artist">{{ truncate(song.artist) }}</p>
            </div>
            <p class="date">{{ formatDate(song.date_added) }}</p>
            <p class="length">{{ formatDuration(song.length) }}</p>
          </div>
          <NuxtLink to="/search" class="add">
            <div class="cover">
              <svg width="36px" height="36px" viewBox="0 0 36 36">
                <path d="M0 0L36 0L36 36L0 36L0 0Z" id="Rectangle" fill="none" fill-rule="evenodd" stroke="none" />
                <path d="M17.4 12L18.6 12L18.6 24L17.4 24L17.4 12Z" id="Rectangle" fill="currentColor"
                  fill-rule="evenodd" stroke="none" />
                <path d="M24 17.4L24 18.6L12 18.6L12 17.4L24 17.4Z" id="Rectangle" fill="currentColor"
                  fill-rule="evenodd" stroke="none" />
              </svg>
            </div>
            <div class="title">Add Songs</div>
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useMusicStore } from '~/stores/music';
import { computed, ref, watch, onMounted } from "vue";
import type { Song, Playlist } from '~/types/types';
import { open } from '@tauri-apps/plugin-dialog';

const { $music } = useNuxtApp();
const route = useRoute();
const playlistId = route.params.playlist.toString();

const musicStore = useMusicStore();
const playlist = computed<Playlist | null>(() => musicStore.getPlaylistByID(playlistId));
const playlistName = ref<string>(playlist.value?.name || '');
const searchQuery = ref<string>('');

const songsDetails = ref<Song[]>([]);

const playlistCover = ref("");

onMounted(async () => {
  playlistCover.value = await $music.searchCoverByPlaylistId(playlistId);
  if (playlist.value?.songs) {
    songsDetails.value = await Promise.all(playlist.value.songs.map(songId => {
      const songDetail = musicStore.getSongByID(songId);
      return songDetail ? songDetail : null;
    }).filter((song): song is Song => song !== null));
  }
});

const filteredSongs = computed<Song[]>(() => {
  const songs = songsDetails.value;
  if (!searchQuery.value.trim()) {
    return songs.sort((a, b) => new Date(b.date_added).getTime() - new Date(a.date_added).getTime());
  }
  return songs.filter(song =>
    song.title.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    song.artist.toLowerCase().includes(searchQuery.value.toLowerCase())
  ).sort((a, b) => new Date(b.date_added).getTime() - new Date(a.date_added).getTime());
});

const songsCountAndDuration = computed<string>(() => {
  if (!playlist.value || playlist.value.songs.length === 0) {
    return '0 songs, 0 min';
  }
  const totalDuration = songsDetails.value.reduce((acc, song) => acc + song.length, 0);
  const hours = Math.floor(totalDuration / 3600);
  const minutes = Math.floor((totalDuration % 3600) / 60);
  const seconds = totalDuration % 60;
  const hoursPart = hours > 0 ? `${hours} hr ` : '';
  const minutesPart = `${minutes} min`;
  const secondsPart = seconds > 0 ? ` ${seconds} sec` : '';
  return `${songsDetails.value.length} song${songsDetails.value.length > 1 ? 's' : ''}, ${hoursPart}${minutesPart}${secondsPart}`;
});

const updatePlaylistName = () => {
  if (playlist.value?.name !== playlistName.value) {
    musicStore.renamePlaylist(playlistId, playlistName.value);
  }
};

watch(playlist, (newVal) => {
  playlistName.value = newVal?.name || '';
});

async function playSong(songId: string) {
  if (!playlist.value || playlist.value.songs.length === 0) {
    console.error("No songs in playlist.");
    return;
  }
  const startIndex = filteredSongs.value.findIndex(song => song.id === songId);
  const queueIds = [...filteredSongs.value.slice(startIndex), ...filteredSongs.value.slice(0, startIndex)].map(song => song.id);
  await $music.setQueue(queueIds);
  await $music.setSong(songId);
  $music.play();
}

function truncate(text: string | null | undefined, length: number = 45): string {
  if (text == null) return '';
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

async function selectNewCover() {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        { name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif'] },
      ],
      directory: false,
    });
    if (selected) {
      await $music.updatePlaylistCover(playlistId, selected);
      playlistCover.value = await $music.searchCoverByPlaylistId(playlistId);
    }
  } catch (error) {
    console.error('Error selecting new cover:', error);
  }
}
</script>

<style scoped lang="scss">
@import "~/assets/styles/pages/playlist.scss";
</style>
