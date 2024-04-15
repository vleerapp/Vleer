<template>
  <p class="library-title">Library</p>
  <div class="library element">
    <div class="search-container">
      <IconsSearch />
      <input class="input" spellcheck="false" v-model="searchQuery" />
    </div>
    <div class="items">
      <div
        v-for="song in filteredSongs"
        :key="song.id"
        @click="play(song.id)"
        class="song"
      >
        <img :src="song.coverURL" :alt="song.title" class="cover" />
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
