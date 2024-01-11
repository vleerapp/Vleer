<template>
  <div>
    <input
      type="text"
      v-model="searchTerm"
      @keyup.enter="searchSongs"
      placeholder="Search for songs"
    />
    <ul v-if="searchResults.length > 0">
      <li
        v-for="(song, index) in searchResults"
        :key="song.videoId"
        :class="{ 'first-result': index === 0 }"
      >
        <a :href="song.watchUrl" target="_blank">
          <img :src="song.thumbnail" :alt="song.title" class="cover-image" />
          <div>{{ song.title }}</div>
        </a>
      </li>
    </ul>
  </div>
</template>

<script setup>
import { ref } from "vue";

const searchTerm = ref("");
const searchResults = ref([]);

const searchSongs = async () => {
  if (searchTerm.value.trim() === "") {
    searchResults.value = [];
    return;
  }

  const { data, error } = await useFetch("/api/search", {
    params: {
      term: searchTerm.value,
    },
  });

  if (error.value) {
    console.error("Failed to fetch search results:", error.value);
    searchResults.value = [];
  } else {
    searchResults.value = data.value.map((item) => ({
      ...item,
      watchUrl: `https://www.youtube.com/watch?v=${item.videoId}`,
    }));
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
