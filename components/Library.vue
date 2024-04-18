<template>
  <div class="library element">
    <p class="element-title">Library</p>
    <div class="top">
      <p class="link">
        <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns:xlink="http://www.w3.org/1999/xlink"
          xmlns="http://www.w3.org/2000/svg">
          <g id="Icon">
            <path d="M0 0L16 0L16 16L0 16L0 0Z" id="Rectangle" fill="none" fill-rule="evenodd" stroke="none" />
            <path d="M2 14L2 2L3 2L3 14L2 14ZM5.5 14L5.5 2L6.5 2L6.5 14L5.5 14ZM8.8 14L8.8 2L13.8 4.5L13.8 14L8.8 14Z"
              id="Rectangle-3-Union" fill-rule="evenodd" stroke="none" />
          </g>
        </svg>
        Your Library
      </p>
      <button @click="createAndOpenPlaylist" class="create-playlist">
        <IconsAdd />
      </button>
    </div>
    <div class="search-container">
      <IconsSearch />
      <input class="input" spellcheck="false" placeholder="Search Playlists" v-model="searchQuery" />
    </div>
    <div class="items">
      <div v-for="playlist in filteredPlaylists" :key="playlist.id" @click="openPlaylist(playlist.id)" class="song">
        <img :src="playlist.cover" class="cover">
        <div class="info">
          <p class="title">{{ truncate(playlist.name) }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { type Playlist } from "~/types/types";
import { ref, onMounted, watch, computed } from "vue";
import { useMusicStore } from "~/stores/music";
import { useRouter } from 'vue-router';
import { v4 as uuidv4 } from 'uuid';

const { $music } = useNuxtApp();
const musicStore = useMusicStore();
const router = useRouter();

const searchQuery = ref("");
const playlists = ref([]);

async function fetchPlaylists() {
  const rawPlaylists = Object.values(musicStore.getSongsData().playlists);
  const playlistsWithCovers = await Promise.all(rawPlaylists.map(async playlist => {
    const cover = await $music.searchCoverByPlaylistId(playlist.id);
    return { ...playlist, cover: cover || '/cover.png' };
  }));
  playlists.value = playlistsWithCovers;
}

const filteredPlaylists = computed(() => {
  return playlists.value.filter(playlist =>
    playlist.name && playlist.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

onMounted(fetchPlaylists);

watch(() => musicStore.getSongsData().playlists, async () => {
  await fetchPlaylists(); // Re-fetch playlists when any change is detected in the store
}, { deep: true });

function openPlaylist(playlistId: string) {
  router.push(`/${playlistId}`);
}

function truncate(text: string, length: number = 45) {
  return text.length > length ? text.substring(0, length - 3).trim() + "..." : text;
}

async function createAndOpenPlaylist() {
  const newPlaylistId = uuidv4();
  const newPlaylist = {
    id: newPlaylistId,
    name: 'New Playlist',
    date: new Date().toLocaleDateString(),
    cover: '/cover.png',
    songs: []
  };
  musicStore.createPlaylist(newPlaylist);
  await fetchPlaylists();
  router.push(`/${newPlaylistId}`);
}
</script>

<style lang="scss">
@import "~/assets/styles/components/library.scss";
</style>