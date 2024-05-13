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
            <img :src="song.coverURL || '/cover.png'" :alt="song.title" class="cover" />
            <div class="titles">
              <p class="title">{{ truncate(song.title) }}</p>
              <p class="artist">{{ truncate(song.artist) }}</p>
            </div>
            <p class="date">{{ formatDate(song.date_added) }}</p>
            <p class="length">{{ formatDuration(song.length) }}</p>
          </div>
          <div v-if="!addSongs" @click="toggleAddSongs" class="add">
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
          </div>
        </div>
        <div v-if="addSongs" class="addsongs">
          <div class="horizontal">
            <div>
              <p>Search for songs to add</p>
              <div class="search-container">
                <IconsSearch />
                <input class="input" placeholder="Search for songs to add" spellcheck="false" v-model="addSearchQuery"
                  @input="addSearchQuery = $event.target.value" />
              </div>
            </div>
            <svg @click="toggleAddSongs" width="32px" height="32px" viewBox="0 0 32 32" class="close">
              <g id="add">
                <path d="M0 0L32 0L32 32L0 32L0 0Z" id="Rectangle" fill="none" fill-rule="evenodd" stroke="none" />
                <path d="M23.6367 6.66626L25.334 8.3634L8.36328 25.334L6.66626 23.6368L23.6367 6.66626Z" id="Rectangle"
                  fill="currentColor" fill-rule="evenodd" stroke="none" />
                <path d="M25.3339 23.6366L23.6368 25.3336L6.66626 8.3631L8.36328 6.66602L25.3339 23.6366Z"
                  id="Rectangle" fill="currentColor" fill-rule="evenodd" stroke="none" />
              </g>
            </svg>
          </div>
          <div class="songs">
            <div v-for="(song, index) in addFilteredSongs" :key="song.id" @click="addSongToPlaylist(song)" class="song">
              <img :src="song.coverURL" :alt="song.title" class="cover" />
              <div class="titles">
                <p class="title">{{ truncate(song.title) }}</p>
                <p class="artist">{{ truncate(song.artist) }}</p>
              </div>
              <p class="lenght">{{ formatDuration(song.length) }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import { computed, ref, watch, onMounted } from "vue";
import type { Song, Playlist } from '~/types/types';
import { open } from '@tauri-apps/plugin-dialog';

const { $music } = useNuxtApp();
const route = useRoute();
const playlistId = route.params.playlist.toString();

const playlist = ref<Playlist | null>(null);
const playlistName = ref<string>(playlist.value?.name || '');
const searchQuery = ref<string>('');

const songsDetails = ref<Song[]>([]);
const playlistCover = ref("/cover.png");
const addSongs = ref(false);

const songs = ref<Song[]>([]); // for loading songs for the add search
const addSearchQuery = ref(""); // for loading songs for the add search
const addFilteredSongs = ref<Song[]>([]); // for loading songs for the add search

function toggleAddSongs() {
  addSongs.value = !addSongs.value;
}

watchEffect(async () => {
  playlist.value = $music.getPlaylistByID(playlistId);
});

onMounted(async () => {
  playlistCover.value = await $music.getCoverURLFromID(playlistId);
  if (playlist.value?.songs) {
    songsDetails.value = playlist.value.songs.map(songId => {
      const songDetail = $music.getSongByID(songId);
      return songDetail ? songDetail : null;
    }).filter((song): song is Song => song !== null);
  }

  await loadSongs(); // for loading songs for the add search
  await fetchPlaylist();
});


// for loading songs for the add search
const loadSongs = async () => {
  const loadedSongs = $music.getSongs();
  await Promise.all(
    loadedSongs.map(async (song) => {
      song.coverURL = await $music.getCoverURLFromID(song.id);
    })
  );
  songs.value = loadedSongs;
};

watch(addSearchQuery, async (newValue) => {
  if (newValue.trim() === '') {
    addFilteredSongs.value = [];
    return;
  }

  const localFilteredSongs = songs.value.filter((song) =>
    song.title.toLowerCase().includes(newValue.toLowerCase()) ||
    song.artist.toLowerCase().includes(newValue.toLowerCase())
  );

  addFilteredSongs.value = localFilteredSongs;

  // if (localFilteredSongs.length > 0) {
  //   addFilteredSongs.value = localFilteredSongs;
  // } else {
  //   try {
  //     const response = await fetch(`https://pipedapi.wireway.ch/search?q=${newValue}&filter=music_songs`);
  //     const data = await response.json();
  //     const fetchedSongs = data.items
  //       .filter(item => item.type !== 'channel')
  //       .map(item => ({
  //         id: item.url.match(/(?:\/watch\?v=)([^&]+)/)[1],
  //         title: item.title,
  //         artist: item.uploaderName,
  //         length: item.duration,
  //         coverURL: item.thumbnail
  //       }));
  //     addFilteredSongs.value = fetchedSongs;
  //   } catch (error) {
  //     console.error("Failed to fetch songs:", error);
  //     addFilteredSongs.value = [];
  //   }
  // }
}, { immediate: true });

async function addSongToPlaylist(song: Song) {
  $music.addSongToPlaylist(playlistId, song.id);

  await fetchPlaylist();
}

//////////////////////////////////////// for playlist songs

async function fetchPlaylist() {
  playlist.value = $music.getPlaylistByID(playlistId);
}

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
    $music.renamePlaylist(playlistId, playlistName.value);
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
