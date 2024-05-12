<template>
  <div class="main element">
    <p class="element-title">Home</p>
    <div class="index">
      <pre class="ascii">
                __                        
 _      _____  / /________  ____ ___  ___ 
| | /| / / _ \/ / ___/ __ \/ __ `__ \/ _ \
| |/ |/ /  __/ / /__/ /_/ / / / / / /  __/
|__/|__/\___/_/\___/\____/_/ /_/ /_/\___/ 
      </pre>

      <div class="playlists">
        <div class="title">Playlists</div>
        <div class="cards" ref="playlist_cards">
          <template v-for="n in 6">
            <div v-if="sortedPlaylists.value && sortedPlaylists.value.length > n" :key="sortedPlaylists.value[n].id"
              class="playlist">
              <NuxtLink :to="'/' + sortedPlaylists.value[n].id">
                <img :src="sortedPlaylists.value[n].coverURL" height="64px" alt="playlist cover" class="cover">
                <p class="name">{{ truncate(sortedPlaylists.value[n].name) }}</p>
              </NuxtLink>
              <button class="play">
                <svg width="10.5px" height="14px" viewBox="0 0 10.5 14" version="1.1"
                  xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg">
                  <g id="Group">
                    <path d="M0 14L0 0L10.5 7L0 14Z" id="Shape" fill="#000000" stroke="none" />
                  </g>
                </svg>
              </button>
            </div>
            <div v-else :key="n" class="playlist placeholder">
              <img src="/cover.png" height="64px" alt="loading" class="cover">
              <p class="name">Loading...</p>
              <button class="play">
                <svg width="10.5px" height="14px" viewBox="0 0 10.5 14" version="1.1"
                  xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg">
                  <g id="Group">
                    <path d="M0 14L0 0L10.5 7L0 14Z" id="Shape" fill="#000000" stroke="none" />
                  </g>
                </svg>
              </button>
            </div>
          </template>
        </div>
      </div>

      <div class="recently-played">
        <div class="title">Recently played</div>
        <div class="cards" ref="song_cards">
          <div v-for="song in sortedRecentlyPlayed" :key="song.id" @click="play(song.id)" class="song">
            <img :src="song.coverURL" :alt="song.title" class="cover" />
            <div class="info">
              <p class="title" :title="song.title">{{ song.title }}</p>
              <p class="artist" :title="song.artist">{{ song.artist }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
const { $music } = useNuxtApp();

export default {
  data() {
    return {
      playlists: [],
      songs: [],
      playlist_cards: ref(null),
      song_cards: ref(null),
      maxCards: ref(5),
      cardsWidth: ref(0),
      cardMinWidth: 180,
      cardMaxWidth: 238,
      cardGap: 16
    }
  },
  methods: {
    updateWidthSongs() {
      if (this.$refs.song_cards) {
        const clientWidth = this.$refs.song_cards.clientWidth;
        this.cardsWidth.value = clientWidth;
        this.updateMaxCardsDirect(clientWidth);
      }
    },
    updateMaxCardsDirect(clientWidth) {
      if (clientWidth > 0) {
        const maxPossible = Math.floor(clientWidth / (this.cardMinWidth + this.cardGap));
        this.maxCards.value = maxPossible;
      }
    },
    updateWidthPlaylists() {
      if (this.$refs.playlist_cards) {
        const width = this.$refs.playlist_cards.clientWidth;
        const final = Math.round((width - 32) / 3);
        this.$refs.playlist_cards.style.gridTemplateColumns = `repeat(3, ${final}px)`;
      }
    },
    play(id) {
      this.$music.setSong(id).then(() => {
        this.$music.play();
      });
    },
    truncate(text, length = 24) {
      return text.length > length ? text.substring(0, length - 3) + '...' : text;
    }
  },
  computed: {
    sortedPlaylists() {
      return this.playlists.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
    },
    sortedRecentlyPlayed() {
      return this.songs.filter(song => song.last_played)
        .sort((a, b) => new Date(b.last_played).getTime() - new Date(a.last_played).getTime())
        .slice(0, this.maxCards);
    }
  },
  mounted() {
    this.playlists = $music.getPlaylists();
    this.songs = $music.getSongs();
    // this.updateWidthSongs();
    // this.updateWidthPlaylists();

    console.log(this.sortedPlaylists.value);

    window.addEventListener('resize', this.updateWidthSongs);
    window.addEventListener('resize', this.updateWidthPlaylists);
  }
}
</script>

<style scoped lang="scss">
@import "~/assets/styles/pages/index.scss";
</style>
