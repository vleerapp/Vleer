<template>
  <div class="main element">
    <p class="element-title">Create Playlist</p>
    <div class="create-playlist">
      <h1>Create Playlist</h1>
      <input v-model="playlistName" placeholder="Playlist Name" />
      <button @click="createPlaylist">Create</button>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useMusicStore } from '~/stores/music';
import { v4 as uuidv4 } from 'uuid';

const playlistName = ref('');
const musicStore = useMusicStore();

function createPlaylist() {
  if (!playlistName.value) {
    alert('Please enter a playlist name');
    return;
  }
  const newPlaylist = {
    id: uuidv4(),
    name: playlistName.value,
    songs: [],
  };
  musicStore.createPlaylist(newPlaylist);
  playlistName.value = '';
  alert('Playlist created successfully');
}
</script>

<style scoped lang="scss">
@import "~/assets/styles/pages/create-playlist.scss";
</style>