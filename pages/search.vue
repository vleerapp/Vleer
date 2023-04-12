<template>
  <NuxtLayout name="search">
    <input type="text" v-model="searchQuery" ref="searchInput" class="search" placeholder="Search" />
    <h1 style="display: none;">Search Results for "{{ formattedSearchQuery }}"</h1>
    <br>
    <ul>
      <li class="searchResults" :class="index === 0 ? 'bigResult' : ''" v-for="(result, index) in searchResults"
        :key="index">
        <div class="searchResult">
          <img class="searchResultCover" :src="result.coverURL" />
          <div class="searchResultName"> {{ result.name }}</div>
          <div class="searchResultLenght"> {{ result.lenght }}</div>
          <div class="searchResultArtist">{{ result.artist }}</div>
        </div>
      </li>
    </ul>
  </NuxtLayout>
</template>

<script>
const itunes = "https://itunes.apple.com/search?";
const itunesLookup = "https://itunes.apple.com/lookup?";

const apiKey = "AIzaSyA5tnrbbwA_Z-ckEq-E5vgQZ7IvcDojQ_k";
const maxResults = 10;

class Song {
  constructor(json) {
    if ("results" in json) {
      try {
        json = json["results"][0];
      } catch { }
    }
    this.kind = json["kind"];
    this.artistName = json["artistName"];
    this.collectionName = json["collectionName"];
    this.trackName = json["trackName"];
    this.artistViewUrl = json["artistViewUrl"];
    this.collectionViewUrl = json["collectionViewUrl"];
    this.trackViewUrl = json["trackViewUrl"];
    this.image = json["artworkUrl100"];
    this.releaseDate = json["releaseDate"];
    this.collectionExplicitness =
      json["collectionExplicitness"] == "notExplicit" ? false : true;
    this.trackExplicitness =
      json["trackExplicitness"] == "notExplicit" ? false : true;
    this.discCount = json["discCount"];
    this.discNumber = json["discNumber"];
    this.trackCount = json["trackCount"];
    this.trackTimeMillis = json["trackTimeMillis"];
    this.country = json["country"];
    this.primaryGenreName = json["primaryGenreName"];
    this.isStreamable = json["isStreamable"].toString();
    this.artistId = json["artistId"];
    this.collectionId = json["collectionId"];
    this.trackId = json["trackId"];
  }

  getImage() {
    return this.image;
  }

  getName() {
    return this.trackName;
  }

  getArtistName() {
    return this.artistName;
  }

  getCountry() {
    return this.country;
  }

  getTrackViewUrl() {
    return this.trackViewUrl;
  }

  isStreamable() {
    return this.isStreamable;
  }

  getCollectionName() {
    return this.collectionName;
  }

  getResizedImage(size) {
    return resizeImage(this.image, size);
  }

  getIds() {
    return [this.trackId, this.collectionId, this.artistId];
  }

  getLength() {
    return this.trackTimeMillis;
  }

  getLengthNormal() {
    const totalSeconds = Math.floor(this.trackTimeMillis / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes}:${seconds.toString().padStart(2, "0")}`;
  }

  isExplicit() {
    return this.trackExplicitness;
  }

  searchForSongName(country = "CH", limit = 50, explicit = true) {
    return get(this.trackName, country, limit, explicit);
  }

  videoURL() {}
}

function resizeImage(url, size) {
  url = url.replace("100x100bb.jpg", `${size}x${size}bb.jpg`);
  url = url.replace("60x60bb.jpg", `${size}x${size}bb.jpg`);
  url = url.replace("30x30bb.jpg", `${size}x${size}bb.jpg`);
  return url;
}

function get(term, country = "CH", limit = 100, explicit = true) {
  const apiUrl = `${itunes}term=${term}&media=music&entity=song&country=${country}&limit=${limit}&explicit=${
    explicit ? "Yes" : "No"
  }&attribute=ratingIndex`;
  return fetch(apiUrl)
    .then((response) => response.json())
    .then((data) => {
      const songList = [];
      data.results.forEach((item) => {
        var song = new Song(item);
        songList.push(song);
      });
      return songList;
    });
}

export default {
  data() {
    console;
    return {
      searchQuery: this.$route.query.q || "",
      searchResults: [],
    };
  },
  created() {
    this.search();
    this.searchQuery = ""
  },
  watch: {
    "$route.query.q": function () {
      this.searchQuery = this.$route.query.q || "";
      this.search();
    },
  },
  mounted() {
    this.$nextTick(() => {
      this.$refs.searchInput.focus();
    });
  },
  computed: {
    formattedSearchQuery() {
      get(this.searchQuery).then((songList) => {
        this.searchResults = [];
        songList.forEach((item) => {
          this.searchResults.push({
            name: (item.getName().length > 20) ? (item.getName().slice(0, 20).at(-1) == " ") ? item.getName().slice(0, 19) + '...' : item.getName().slice(0, 20) + '...' : item.getName(),
            artist: (item.getArtistName().length > 30) ? (item.getArtistName().slice(0, 30).at(-1) == " ") ? item.getArtistName().slice(0, 29) + '...' : item.getArtistName().slice(0, 30) + '...' : item.getArtistName(),
            lenght: item.getLengthNormal(),
            coverURL: item.getResizedImage(512),
          });
        });
      });
      return this.searchQuery ? this.searchQuery : "";
    },
  },
  methods: {
    search() { },
  },
};
</script>

<style>
@import '~/css/search.css';
</style>