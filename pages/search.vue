<template>
  <NuxtLayout name="search">
    <input type="text" v-model="searchQuery" ref="searchInput" class="search" placeholder="Search" />
    <h1>Search Results for "{{ formattedSearchQuery }}"</h1>
    <ul>
      <li v-for="(result, index) in searchResults" :key="index">{{ result }}</li>
    </ul>
  </NuxtLayout>
</template>

<script>
export default {
  computed: {
    searchQuery() {
      return this.$route.query.q || '';
    },
    searchResults() {
      // Perform search based on searchQuery and return the results
      // This is just a placeholder code. You'll need to implement the search functionality
      // that fits your app's needs.
      return ['Result 1', 'Result 2', 'Result 3'];
    },
    formattedSearchQuery() {
      if (this.searchQuery === '') {
        return ' ';
      } else {
        return `"${this.searchQuery}"`;
      }
    }
  },
  watch: {
    searchQuery: function (newQuery) {
      this.$router.push({ path: '/search', query: { q: newQuery } });
    }
  },
  mounted() {
    this.$nextTick(() => {
      this.$refs.searchInput.focus();
    });
  }
};
</script>