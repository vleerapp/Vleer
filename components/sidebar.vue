<template>
  <div class="sidebar" id="sidebar">
    <div class="minibar">
      <img src="~/assets/waradu.png" alt="userimg" class="user-img" />
      <nuxt-link class="settings" to="/settings">
        <img src="/svg/linear/setting-2.svg" width="18" height="18" />
      </nuxt-link>
    </div>
    <div id="sidenav_tabs">
      <div class="tab">
        <nuxt-link to="/" class="tab-link">
          <div style="--img: url('/svg/linear/home-2.svg')" class="tab_icon home_icon"></div>
          <span class="tab_text">Home</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/songs" class="tab-link">
          <div style="--img: url('/svg/linear/music.svg')" class="tab_icon songs_icon"></div>
          <span class="tab_text">Songs</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/playlists" class="tab-link">
          <div style="--img: url('/svg/linear/note.svg')" class="tab_icon playlists_icon"></div>
          <span class="tab_text">Playlists</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/library" class="tab-link">
          <div style="--img: url('/svg/linear/video-square.svg')" class="tab_icon library_icon"></div>
          <span class="tab_text">Library</span>
        </nuxt-link>
      </div>
    </div>
    <div id="minimizer" style="--arrow: url('/svg/linear/arrow-close.svg')" v-on:click="toggleSidebar"
      class="minimizeSidebar" alt=""></div>
  </div>
</template>

<script>
function toggleSidebar() {
  var element = document.getElementById("sidebar");
  var content = document.getElementById("content");
  var minimizer = document.getElementById("minimizer");
  element.classList.toggle("minimized");
  minimizer.classList.toggle("mini");
  content.classList.toggle("contentMinimized");
}
export default {
  computed: {},
  methods: {
    toggleSidebar,
  },
};
</script>

<style>
@import "~/css/sidebar.css";
@import "~/css/media.css";
</style>
