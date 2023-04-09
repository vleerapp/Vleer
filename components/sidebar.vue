<template>
  <div class="sidebar">
    <div class="minibar">
      <img src="~/assets/waradu.png" alt="userimg" class="user-img">
      <div class="settings">
        <img src="~/assets/svg/linear/setting-2.svg" width="18" height="18">
      </div>
    </div>
    <div id="sidenav_tabs">
      <div class="tab">
        <nuxt-link to="/" class="tab-link">
          <img src="../assets/svg/bold/home-2.svg" class="tab_icon">
          <span class="tab_text">Home</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/songs" class="tab-link">
          <img src="../assets/svg/linear/music.svg" class="tab_icon">
          <span class="tab_text">Songs</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/playlists" class="tab-link">
          <img src="../assets/svg/linear/note.svg" class="tab_icon">
          <span class="tab_text">Playlists</span>
        </nuxt-link>
      </div>
      <div class="tab">
        <nuxt-link to="/library" class="tab-link">
          <img src="../assets/svg/linear/video-square.svg" class="tab_icon">
          <span class="tab_text">Library</span>
        </nuxt-link>
      </div>
    </div>
  </div>
</template>

<style>
@import '~/css/sidebar.css';
@import '~/css/media.css';
</style>