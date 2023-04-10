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
  data() {
    return {
      searchQuery: this.$route.query.q || '',
      searchResults: []
    }
  },
  created() {
    this.search();
  },
  watch: {
    '$route.query.q': function () {
      this.searchQuery = this.$route.query.q || '';
      this.search();
    }
  },
  mounted() {
    this.$nextTick(() => {
      this.$refs.searchInput.focus();
    });
  },
  computed: {
    formattedSearchQuery() {
      return this.searchQuery ? this.searchQuery.charAt(0).toUpperCase() + this.searchQuery.slice(1) : '';
    }
  },
  methods: {
    search() {
      // Perform search based on searchQuery and update the searchResults array
      // This is just a placeholder code. You'll need to implement the search functionality
      // that fits your app's needs.
      this.searchResults = ['Result 1', 'Result 2', 'Result 3'];
    }
  }
};
</script>