<template>
  <div>
    <input
      type="text"
      v-model="searchTerm"
      @keyup.enter="searchSongs()"
      :disabled="isLoading"
      placeholder="Search for songs"
    />
    <ul v-if="searchResults.length > 0">
      <li
        v-for="(song, index) in searchResults"
        :key="song.trackId"
        :class="{ 'first-result': index === 0 }"
      >
        <a @click.prevent="handleSongClick(song)" :href="song.youtubeLink || 'javascript:void(0);'" target="_blank">
          <img :src="song.artworkUrl100" alt="Cover image" />
          <div>{{ song.trackName }}</div>
          <div>{{ song.artistName }}</div>
        </a>
      </li>
    </ul>
    <input
      type="text"
      v-model="url"
      @keyup.enter="downloadSong"
      placeholder="Yt url"
    />
  </div>
</template>

<script setup>
import DiscordRPC from "../lib/DiscordRPC";
import Search from "../lib/Search";
import Download from "../lib/Download";
import axios from 'axios';

const searchTerm = ref("");
const searchResults = ref([]);
const isLoading = ref(false);
const url = ref("");

const searchSongs = async () => {
  await DiscordRPC.update(
    "Playing",
    "Searching...",
    "logo",
    "Vleer",
    "search",
    "Search"
  );

  isLoading.value = true;

  if (searchTerm === "") {
    searchResults.value = [];
    isLoading.value = false;
    return;
  }

  try {
    const results = await Search.performSearch(searchTerm.value);
    searchResults.value = results; // Corrected from searchResults.values to searchResults.value
    console.log(searchResults);
  } catch (error) {
    console.error("An unexpected error occurred:", error);
    searchResults.value = [];
  } finally {
    isLoading.value = false;
  }
};

const fetchYoutubeLink = async (song) => {
  if (!song.youtubeLink) {
    try {
      const response = await axios.get(`https://wireway.ch/api/musicAPI/search/?q=${encodeURIComponent(song.trackName + ' ' + song.artistName)}`);
      if (response.data && response.data[0] && response.data[0].url) {
        song.youtubeLink = response.data[0].url; // Assuming the API returns an array and the first object has a url property
      }
    } catch (error) {
      console.error('Error fetching YouTube link:', error);
    }
  }
};

const handleSongClick = async (song) => {
  if (!song.youtubeLink) {
    await fetchYoutubeLink(song);
  }
  if (song.youtubeLink) {
    window.open(song.youtubeLink, '_blank');
  }
};

const downloadSong = async () => {
  const output_path = "C:/Users/pandadev/Desktop/";
  try {
    const filePath = await Download.downloadVideoAsMp3(url.value, output_path);
    console.log("Downloaded MP3 file path:", filePath);
    // Handle the downloaded file path (e.g., show a notification or save dialog)
  } catch (error) {
    console.error("Error:", error);
  }
};
</script>

<style scoped>
.cover-image {
  width: 100px; /* Default thumbnail size */
  height: auto;
}

.first-result .cover-image {
  width: 200px; /* Larger size for the first result */
}

.first-result {
  font-size: 1.2em; /* Larger text for the first result */
}

li {
  list-style-type: none; /* Remove bullet points from list items */
  margin-bottom: 10px; /* Add some space between list items */
}

a {
  text-decoration: none; /* Optional: Removes underline from links */
  color: inherit; /* Optional: Inherits text color from parent */
}

/* Add more styles as needed */
</style>
