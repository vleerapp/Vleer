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
      <NuxtLink class="create-playlist" to="/create-playlist">
        <IconsAdd />
      </NuxtLink>
    </div>
    <div class="search-container">
      <IconsSearch />
      <input class="input" spellcheck="false" v-model="searchQuery" />
    </div>
    <div class="items">
      <div v-for="song in filteredSongs" :key="song.id" @click="play(song.id)" class="song">
        <nuxt-img :src="song.coverURL" :alt="song.title" class="cover" />
        <div class="info">
          <p class="title">{{ truncate(song.title) }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { type Song } from "~/types/types";
import { computed, ref, onMounted, watch } from "vue";
import { useMusicStore } from "~/stores/music";
import { invoke } from "@tauri-apps/api/core";

const { $music } = useNuxtApp();
const musicStore = useMusicStore();

const songs = ref<Song[]>([]);
const searchQuery = ref("");

onMounted(async () => {
  await loadSongs();
});

const loadSongs = async () => {
  const loadedSongs = await $music.getSongs();
  const songArray = Object.values(loadedSongs.songs);
  await Promise.all(
    songArray.map(async (song) => {
      song.coverURL = await $music.getCoverURLFromID(song.id);
    })
  );
  songs.value = songArray;
};

watch(
  () => musicStore.lastUpdated,
  async () => {
    await loadSongs();
  }
);

const filteredSongs = computed(() => {
  return songs.value
    .filter((song) =>
      song.title.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
    .sort((a, b) => {
      if (searchQuery.value) {
        return (
          a.title.toLowerCase().indexOf(searchQuery.value.toLowerCase()) -
          b.title.toLowerCase().indexOf(searchQuery.value.toLowerCase())
        );
      } else {
        return (
          new Date(b.date_added).getTime() - new Date(a.date_added).getTime()
        );
      }
    })
    .slice(0, 30);
});

async function play(id: string) {
  await $music.setSong(id);
  $music.play();
}

function truncate(text: string, length: number = 45) {
  return text.length > length ? text.substring(0, length - 3).trim() + "..." : text;
}
</script>

<style lang="scss">
@import "~/assets/styles/components/library.scss";
</style>
