<template>
  <div class="library element">
    <p class="element-title">Library</p>
    <div class="search-container">
      <IconsSearch />
      <input class="input" spellcheck="false" placeholder="Search Library" v-model="searchQuery" />
    </div>
    <div class="top">
      <NuxtLink to="/playlists" class="playlists link">
        <div class="playlist-icon">
          <IconsLibraryPlaylistFill v-if="$route.path === '/playlists'" />
          <IconsLibraryPlaylistOutline v-else />
          Playlists
        </div>
        <button @click="createAndOpenPlaylist" class="create-playlist">
          <IconsAdd />
        </button>
      </NuxtLink>
      <NuxtLink to="/liked-songs" class="liked-songs link">
        <IconsLibraryLikedFill v-if="$route.path === '/liked-songs'" />
        <IconsLibraryLikedOutline v-else />
        Liked Songs
      </NuxtLink>
      <NuxtLink to="/songs" class="songs link">
        <IconsLibrarySongsFill v-if="$route.path === '/songs'" />
        <IconsLibrarySongsOutline v-else />
        Songs
      </NuxtLink>
      <NuxtLink to="/albums" class="albums link">
        <IconsLibraryAlbumFill v-if="$route.path === '/albums'" />
        <IconsLibraryAlbumOutline v-else />
        Albums
      </NuxtLink>
      <NuxtLink to="/artists" class="artists link">
        <IconsLibraryArtistFill v-if="$route.path === '/artists'" />
        <IconsLibraryArtistOutline v-else />
        Artists
      </NuxtLink>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useRouter } from 'vue-router';
import { v4 as uuidv4 } from 'uuid';
import type { Playlist } from "~/types/types";

const { $music } = useNuxtApp();
const router = useRouter();

const searchQuery = ref("");

async function createAndOpenPlaylist() {
  const newPlaylistId = uuidv4();
  const newPlaylist: Playlist = {
    id: newPlaylistId,
    name: 'New Playlist',
    date_created: new Date(),
    songs: []
  };
  try {
    await $music.addPlaylist(newPlaylist);
    router.push(`/${newPlaylistId}`);
  } catch (error) {
    console.error("Error creating playlist:", error);
  }
}
</script>

<style lang="scss">
@import "~/assets/styles/components/library.scss";
</style>