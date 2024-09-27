<template>
  <div class="main element">
    <p class="element-title">Search</p>
    <div class="search">
      <div v-if="searchResults.length > 0" class="results">
        <div class="inline">
          <div class="top-result">
            <p>Top Result</p>
            <div @contextmenu.prevent="showContextMenu($event, searchResults[0])" class="content">
              <img :alt="searchResults[0].title" :src="searchResults[0].cover" class="cover" loading="lazy" />
              <div>
                <div class="title">{{ searchResults[0].title }}</div>
                <div class="artist">{{ searchResults[0].artist }}</div>
              </div>
              <div @click="play(searchResults[0])" class="play">
                <svg height="14px" version="1.1" viewBox="0 0 11.083252 14" width="11.083252px" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
                  <path d="M0 0L0 14L11.083252 7L0 0Z" fill="#000000" id="Shape" stroke="none" />
                </svg>
              </div>
              <ContextMenu :menuItems="menuItems" :show="showMenu" :x="menuX" :y="menuY" @close="closeContextMenu" />
            </div>
          </div>

          <div class="songs">
            <p class="songs-title">Songs</p>
            <div class="content">
              <div :class="['song', { playing: isCurrentSong(song) }]" :key="song.id" @contextmenu.prevent="showContextMenu($event, song)" @mouseleave="hoveredSongId = ''" @mouseover="hoveredSongId = song.id" v-for="(song, index) in searchResults.slice(1, 6)">
                <div class="inline-songs">
                  <div @click="play(song)" class="cover">
                    <div class="playing-indicator">
                      <div class="bar"></div>
                      <div class="bar"></div>
                      <div class="bar"></div>
                      <div class="bar"></div>
                    </div>
                    <svg height="14px" version="1.1" viewBox="0 0 14 14" width="14px" v-show="hoveredSongId === song.id" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
                      <g id="Group">
                        <path d="M0 0L14 0L14 14L0 14L0 0Z" fill="none" fill-rule="evenodd" id="Rectangle" stroke="none" />
                        <path d="M2 14L2 0L12.5 7L2 14Z" fill="#FFFFFF" id="Shape" stroke="none" />
                      </g>
                    </svg>
                    <img :alt="song.title" :src="song.cover || '/cover.png'" class="img" />
                  </div>
                  <div class="titles">
                    <p class="title">{{ song.title }}</p>
                    <p class="artist">{{ song.artist }}</p>
                  </div>
                </div>
                <p class="length">{{ formatDuration(song.duration) }}</p>
              </div>
              <ContextMenu :menuItems="menuItems" :show="showMenu" :x="menuX" :y="menuY" @close="closeContextMenu" />
            </div>
          </div>
        </div>

        <div class="albums">
          <div class="section-header">
            <p>Albums</p>
            <div class="scroll-buttons">
              <button :disabled="albumsScrollLeft === 0" @click="scroll('albums', 'left')" class="scroll-button">&lt;</button>
              <button :disabled="albumsScrollLeft >= albumsMaxScroll" @click="scroll('albums', 'right')" class="scroll-button">&gt;</button>
            </div>
          </div>
          <div class="album-grid" ref="albumsGrid">
            <div :key="album.id" class="album-item" v-for="album in albums">
              <img :alt="album.name" :src="album.cover" class="album-cover" />
              <p class="album-title">{{ album.name }}</p>
              <p class="album-artist">{{ album.artist }}</p>
            </div>
          </div>
        </div>

        <div class="playlists">
          <div class="section-header">
            <p>Playlists</p>
            <div class="scroll-buttons">
              <button :disabled="playlistsScrollLeft === 0" @click="scroll('playlists', 'left')" class="scroll-button">&lt;</button>
              <button :disabled="playlistsScrollLeft >= playlistsMaxScroll" @click="scroll('playlists', 'right')" class="scroll-button">&gt;</button>
            </div>
          </div>
          <div class="playlist-grid" ref="playlistsGrid">
            <div :key="playlist.id" class="playlist-item" v-for="playlist in playlists">
              <img :alt="playlist.name" :src="playlist.cover" class="playlist-cover" />
              <p class="playlist-title">{{ playlist.name }}</p>
              <p class="playlist-owner">{{ playlist.artist }}</p>
            </div>
          </div>
        </div>
      </div>
      <div class="no-results" v-else-if="searchTerm">
        No results found for "{{ searchTerm }}"
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { BaseDirectory, exists, writeFile } from '@tauri-apps/plugin-fs'
import { invoke } from '@tauri-apps/api/core'
import axios from 'axios'
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import type { Song } from '~/types/types'

export interface Album {
  artist: string
  artistCover: string
  cover: string
  id: string
  name: string
  songs: Song[]
}

export interface Playlist {
  id: string
  artist: string
  artistCover: string
  cover: string
  name: string
  songs: Song[]
}

export interface ResponseSong {
  id: string
  title: string
  artist: string
  artistCover: string
  album: string
  cover: string
  duration: number
}

const { $music, $player, $settings } = useNuxtApp()

const albums = ref<Album[]>([])
const albumsGrid = ref<HTMLElement | null>(null)
const albumsMaxScroll = ref(0)
const albumsScrollLeft = ref(0)
const currentSearchId = ref(0)
const currentSong = computed(() => $player.currentSong)
const hoveredSongId = ref('')
const menuItems = ref<{ action: () => void; label: string }[]>([])
const menuX = ref(0)
const menuY = ref(0)
const playlists = ref<Playlist[]>([])
const playlistsGrid = ref<HTMLElement | null>(null)
const playlistsMaxScroll = ref(0)
const playlistsScrollLeft = ref(0)
const route = useRoute()
const searchResults = ref<ResponseSong[]>([])
const searchTerm = ref('')
const showMenu = ref(false)

watch(currentSong, () => { })

watch(() => route.query.q, (newQuery) => {
  searchTerm.value = newQuery as string || ''
  if (searchTerm.value) {
    searchSongs()
  } else {
    albums.value = []
    playlists.value = []
    searchResults.value = []
  }
})

onMounted(() => {
  searchTerm.value = route.query.q as string || ''
  if (searchTerm.value) {
    searchSongs()
  }
  updateMaxScroll()
  window.addEventListener('click', closeContextMenu)
  window.addEventListener('resize', updateMaxScroll)
})

async function addToLibrary(song: ResponseSong) {
  try {
    const isLossless = await $settings.getLossless()
    const flacExists = await exists(`Vleer/Songs/${song.id}.flac`, { baseDir: BaseDirectory.Audio })
    const mp3Exists = await exists(`Vleer/Songs/${song.id}.mp3`, { baseDir: BaseDirectory.Audio })

    if ((isLossless && !flacExists) || (!isLossless && !mp3Exists)) {
      const songData: Song = {
        album: '',
        artist: song.artist,
        cover: `/thumbnail?id=${song.id}`,
        date_added: new Date(),
        duration: song.duration,
        id: song.id,
        title: song.title,
      }

      try {
        await invoke('download', { id: song.id, quality: isLossless ? 'lossless' : 'compressed', url: await $settings.getApiUrl() })

        if (!flacExists && !mp3Exists) {
          const response = await axios.get(song.cover, { responseType: 'arraybuffer' })
          const data = new Uint8Array(response.data)
          await writeFile(`Vleer/Covers/${song.id}.png`, data, { baseDir: BaseDirectory.Audio })
          await $music.addSong(songData)
        }
      } catch (error) {
        console.error('Error downloading video:', error)
        return
      }
    }
  } catch (error) {
    console.error("Failed to handle song play:", error)
  }
}

function closeContextMenu() {
  showMenu.value = false
}

function formatDuration(duration: number) {
  const minutes = Math.floor(duration / 60)
  const seconds = duration % 60
  return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`
}

function isCurrentSong(song: ResponseSong): boolean {
  return !!currentSong.value && currentSong.value.value?.id === song.id
}

async function play(song: ResponseSong) {
  try {
    const isLossless = await $settings.getLossless()
    const flacExists = await exists(`Vleer/Songs/${song.id}.flac`, { baseDir: BaseDirectory.Audio })
    const mp3Exists = await exists(`Vleer/Songs/${song.id}.mp3`, { baseDir: BaseDirectory.Audio })

    let dbSong = await $music.getSong(song.id)

    if (!dbSong) {
      const songData: Song = {
        album: '',
        artist: song.artist,
        cover: `/thumbnail?id=${song.id}`,
        date_added: new Date(),
        duration: song.duration,
        id: song.id,
        title: song.title,
      }

      await $music.addSong(songData)
      dbSong = songData
    }

    if ((isLossless && !flacExists) || (!isLossless && !mp3Exists)) {
      try {
        await invoke('download', { id: song.id, quality: isLossless ? 'lossless' : 'compressed', url: await $settings.getApiUrl() })

        if (!flacExists && !mp3Exists) {
          const response = await axios.get(song.cover, { responseType: 'arraybuffer' })
          const data = new Uint8Array(response.data)
          await writeFile(`Vleer/Covers/${song.id}.png`, data, { baseDir: BaseDirectory.Audio })
        }
      } catch (error) {
        console.error('Error downloading video:', error)
        return
      }
    }

    await $player.loadSong(dbSong)
    $player.play()

    const apiURL = await $settings.getApiUrl()
    const encodedSearchTerm = encodeURIComponent(searchTerm.value).replace(/[!'()*]/g, escape)
    const updateWeightURL = `${apiURL}/search/update-weight?query=${encodedSearchTerm}&selected_id=${song.id}`
    
    try {
      await fetch(updateWeightURL, {
        method: 'POST',
      })
    } catch (error) {
      console.error('Error updating weight:', error)
    }
  } catch (error) {
    console.error("Failed to handle song play:", error)
  }
}

function scroll(type: 'albums' | 'playlists', direction: 'left' | 'right') {
  const grid = type === 'albums' ? albumsGrid.value : playlistsGrid.value
  if (!grid) return

  const newScrollLeft = direction === 'left'
    ? Math.max(0, grid.scrollLeft - 200)
    : Math.min(grid.scrollLeft + 200, grid.scrollWidth - grid.clientWidth)

  grid.scrollTo({
    behavior: 'smooth',
    left: newScrollLeft,
  })

  if (type === 'albums') {
    albumsScrollLeft.value = newScrollLeft
  } else {
    playlistsScrollLeft.value = newScrollLeft
  }
}

async function searchSongs() {
  if (searchTerm.value === "") {
    albums.value = []
    playlists.value = []
    searchResults.value = []
    return
  }

  const searchId = ++currentSearchId.value

  try {
    const apiURL = await $settings.getApiUrl()

    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), 10000)

    const encodedSearchTerm = encodeURIComponent(searchTerm.value).replace(/[!'()*]/g, escape)

    const response = await fetch(`${apiURL}/search?query=${encodedSearchTerm}&mode=minimal&filter=songs`, {
      signal: controller.signal
    })

    clearTimeout(timeoutId)

    if (searchId !== currentSearchId.value) {
      return
    }

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const data = await response.json()

    albums.value = Object.values(data.albums || {}).map((album: any) => ({
      artist: album.artist,
      artistCover: album.artistCover,
      cover: album.cover.replace("w544-h544", "w160-h160"),
      id: album.id,
      name: album.name,
      songs: album.songs,
    }))

    playlists.value = Object.values(data.playlists || {}).map((playlist: any) => ({
      artist: playlist.artist,
      artistCover: playlist.artistCover,
      cover: playlist.cover.replace("w120-h120", "w160-h160"),
      id: playlist.id,
      name: playlist.name,
      songs: playlist.songs,
    }))

    searchResults.value = Object.values(data.songs || {}).map((song: any) => ({
      id: song.id,
      title: song.title,
      artist: song.artist,
      artistCover: song.artistCover,
      album: song.album,
      cover: song.cover.replace("w544-h544", "w160-h160"),
      duration: song.duration,
    }))

    console.log(albums.value, playlists.value, searchResults.value)
  } catch (error: any) {
    if (error.name === 'AbortError') {
      console.log("Search request timed out")
    } else {
      console.error("Failed to fetch search results:", error, searchTerm.value)
      albums.value = []
      playlists.value = []
      searchResults.value = []
    }
  }
}

function showContextMenu(event: MouseEvent, song: ResponseSong) {
  event.preventDefault()
  menuX.value = event.clientX
  menuY.value = event.clientY
  showMenu.value = true

  menuItems.value = [
    {
      action: () => {
        addToLibrary(song)
      },
      label: 'Add to library',
    },
  ]
}

function updateMaxScroll() {
  if (albumsGrid.value) {
    albumsMaxScroll.value = albumsGrid.value.scrollWidth - albumsGrid.value.clientWidth
  }
  if (playlistsGrid.value) {
    playlistsMaxScroll.value = playlistsGrid.value.scrollWidth - playlistsGrid.value.clientWidth
  }
}

watch([albums, playlists], () => {
  setTimeout(updateMaxScroll, 0)
})
</script>

<style lang="scss" scoped>
@import '~/assets/styles/pages/search.scss';
</style>