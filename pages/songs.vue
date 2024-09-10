<template>
  <div class="main element">
    <p class="element-title">Songs</p>
    <div class="songs">
      <div class="search-container">
        <IconsSearch />
        <input class="input" placeholder="Search" spellcheck="false" v-model="searchQuery" />
      </div>
      <div class="songs-info">
        <div class="cover">#</div>
        <div class="title">Title</div>
        <div class="date">Date added</div>
        <div class="lenght">
          <img src="/Length.svg" alt="" />
        </div>
      </div>
      <div class="items">
        <div v-for="(song, index) in filteredSongs" :key="song.id" @click="play(song.id, index)"
          :class="['song', { playing: currentSong.id === song.id }]" @mouseover="hoveredSongId = song.id"
          @mouseleave="hoveredSongId = ''">
          <div class="cover">
            <div class="playing-indicator">
              <div class="bar"></div>
              <div class="bar"></div>
              <div class="bar"></div>
              <div class="bar"></div>
            </div>
            <svg v-show="hoveredSongId === song.id" width="14px" height="14px" viewBox="0 0 14 14" version="1.1"
              xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg">
              <g id="Group">
                <path d="M0 0L14 0L14 14L0 14L0 0Z" id="Rectangle" fill="none" fill-rule="evenodd" stroke="none" />
                <path d="M2 14L2 0L12.5 7L2 14Z" id="Shape" fill="#FFFFFF" stroke="none" />
              </g>
            </svg>
            <img :src="song.coverURL || '/cover.png'" :alt="song.title" class="img" />
          </div>
          <div class="titles">
            <p class="title">{{ truncate(song.title) }}</p>
            <p class="artist">{{ truncate(song.artist) }}</p>
          </div>
          <p class="date">{{ formatDate(song.date_added) }}</p>
          <p class="lenght">{{ formatDuration(song.duration) }}</p>
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
import { ref, computed, onMounted, watch } from "vue";
import { useNuxtApp } from "#app";
import type { Song } from "~/types/types";

const { $player, $settings, $music } = useNuxtApp();

const searchQuery = ref("");
const songs = ref<Song[]>([]);
const hoveredSongId = ref("");

const fetchSongs = async () => {
  try {
    songs.value = await $music.getSongs();
  } catch (error) {
    console.error("Error fetching songs:", error);
  }
};

onMounted(async () => {
  await fetchSongs();
});

const filteredSongs = computed<Song[]>(() => {
  if (!searchQuery.value.trim()) {
    return songs.value.sort((a, b) => new Date(b.date_added).getTime() - new Date(a.date_added).getTime());
  }
  return songs.value.filter(song =>
    song.title.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    song.artist.toLowerCase().includes(searchQuery.value.toLowerCase())
  ).sort((a, b) => new Date(b.date_added).getTime() - new Date(a.date_added).getTime());
});

async function play(id: string, index: number) {
  const queueSongs = filteredSongs.value.slice(index);
  $settings.setQueue(queueSongs);
  const song = await $music.getSong(id);
  if (song) {
    await $player.loadSong(song);
    $player.play();
  }
}

const currentSong = computed(() => $player.currentSong);

watch(() => $player.paused, (isPaused) => {
  if (!isPaused) {
    visualizer.start();
  } else {
    visualizer.stop();
  }
});

function truncate(text: string, length: number = 45) {
  return text.length > length ? text.substring(0, length - 3).trim() + "..." : text;
}

function formatDate(dateString: string): string {
  const date = new Date(dateString);
  const day = date.getDate().toString().padStart(2, '0');
  const month = (date.getMonth() + 1).toString().padStart(2, '0');
  const year = date.getFullYear();
  return `${day}.${month}.${year}`;
}

function formatDuration(duration: number) {
  const minutes = Math.floor(duration / 60);
  const seconds = duration % 60;
  return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
}

const visualizer = startVisualizer();

watch(() => $player.paused, (isPaused) => {
  if (!isPaused) {
    visualizer.start();
  } else {
    visualizer.stop();
  }
});

watch(() => $player.currentSong, () => {
  if (!$player.paused) {
    visualizer.start();
  } else {
    visualizer.stop();
  }
});

function startVisualizer() {
  let animationFrameId: number;
  const maxHeight = 22;
  const minHeight = 4;
  const pausedHeight = 2;
  const animationSpeed = 0.2;
  const bars = Array.from({ length: 4 }, () => ({
    height: Math.random() * (maxHeight - minHeight) + minHeight,
    direction: Math.random() < 0.5 ? 1 : -1
  }));

  function draw() {
    const barElements = document.querySelectorAll('.playing-indicator .bar');
    bars.forEach((bar, index) => {
      bar.height += bar.direction * animationSpeed;
      if (bar.height > maxHeight || bar.height < minHeight) {
        bar.direction *= -1;
      }
      (barElements[index] as HTMLElement).style.height = `${bar.height}px`;
    });
    animationFrameId = requestAnimationFrame(draw);
  }

  function stop() {
    cancelAnimationFrame(animationFrameId);
    const barElements = document.querySelectorAll('.playing-indicator .bar');
    barElements.forEach((bar) => {
      (bar as HTMLElement).style.height = `${pausedHeight}px`;
    });
  }

  let isPlaying = false;

  function toggleVisualizer(play: boolean) {
    isPlaying = play;
    if (isPlaying) {
      draw();
    } else {
      stop();
    }
  }

  return {
    start: () => toggleVisualizer(true),
    stop: () => toggleVisualizer(false)
  };
}
</script>

<style scoped lang="scss">
@import "~/assets/styles/pages/songs.scss";
</style>