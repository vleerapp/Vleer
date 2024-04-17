<template>
  <div class="main element">
    <p class="element-title">Songs</p>
    <div class="songs">
      <div class="songs-info">
        <div class="cover">#</div>
        <div class="title">Title</div>
        <div class="date">Date added</div>
        <div class="lenght">
          <img src="/Lenght.svg" alt="">
        </div>
      </div>
      <div class="items">
        <div v-for="song in filteredSongs" :key="song.id" @click="play(song.id)" class="song">
          <img :src="song.coverURL" :alt="song.title" class="cover" />
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
</script>

<style scoped lang="scss">
@import "~/assets/styles/pages/songs.scss";
</style>