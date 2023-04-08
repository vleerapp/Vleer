<template>
  <div class="titlebar">
    <div class="titlebar-drag"></div>
    <div class="titlebar-close"></div>
    <div class="titlebar-minimize"></div>
    <div class="titlebar-maximize"></div>
  </div>
  <div id="page" class="page">
    <div id="sidebar" class="sidebar">
      <div id="sidenav_top" class="top">
        <div class="user">
          <img class="userimg" id="userimg" onclick="cmMenu();" src="images/waradu.png" alt="">
          <div class="greetings">Hello,<br>Waradu</div>
          <img onclick="toggleSidebar();" class="minimizeSidebar" src="images/svg/linear/close-open.svg" alt="">
        </div>
      </div>
      <div id="sidenav_tabs" class="tabs">
        <div class="tab active"><a href="#" class="tab-link">
            <div class="tab-icon-div"><img src="images/svg/bold/home-2.svg" class="tab_icon"></div><span
              class="tab_text">Home</span>
          </a></div>
        <div class="tab"><a href="#" class="tab-link">
            <div class="tab-icon-div"><img src="images/svg/linear/music.svg" class="tab_icon"></div><span
              class="tab_text">Songs</span>
          </a></div>
        <div class="tab"><a href="#" class="tab-link">
            <div class="tab-icon-div"><img src="images/svg/linear/note.svg" class="tab_icon"></div><span
              class="tab_text">Playlists</span>
          </a></div>
        <div class="tab"><a href="#" class="tab-link">
            <div class="tab-icon-div"><img src="images/svg/linear/video-square.svg" class="tab_icon"></div><span
              class="tab_text">Libary</span>
          </a></div>
      </div>
      <div id="sidenav_bottom">
        <div class="quick-actions">
          <div style="--bgsrc: url(../images/waradu.png);" class="quick-actions-placeholder">
            <div class="quick-action-inside"><img src="images/svg/bold/play.svg" alt=""></div>
          </div>
          <div style="--bgsrc: url(../images/aqua.png);" class="quick-actions-placeholder">
            <div class="quick-action-inside"><img src="images/svg/bold/play.svg" alt=""></div>
          </div>
          <div style="--bgsrc: url(../images/violett.png);" class="quick-actions-placeholder">
            <div class="quick-action-inside"><img src="images/svg/bold/play.svg" alt=""></div>
          </div>
          <div style="--bgsrc: url(../images/santa.png);" class="quick-actions-placeholder">
            <div class="quick-action-inside"><img src="images/svg/bold/play.svg" alt=""></div>
          </div>
        </div>
        <div class="credits">Made By <a target="_blank" href="https://portfolio-0pandadev.vercel.app/"
            class="credit_link">PandaDEV</a> and <a target="_blank" href="https://waradu.tk"
            class="credit_link">Waradu</a>. See project at <a target="_blank" href="https://github.com/Waradu"
            class="credit_link">Github</a></div>
      </div>
    </div>
    <div id="main-page" class="main-page">
      <input type="text" class="search" placeholder="Search" />

      <div id="main" class="main">
        <div class="tab-description">Home</div>
        <div class="recomendations">
          <div class="recomentation">
            <img src="images/violett.png" alt="" class="rec-image">
            <div class="rec-text">
              <div class="rec-name">Violett</div>
              <div class="rec-artist">Connor Price & Killa</div>
            </div>
          </div>
          <div class="recomentation">
            <img src="images/aqua.png" alt="" class="rec-image">
            <div class="rec-text">
              <div class="rec-name">Aqua</div>
              <div class="rec-artist">Ugo Melone</div>
            </div>
          </div>
          <div class="recomentation">
            <img src="images/nothing.png" alt="" class="rec-image">
            <div class="rec-text">
              <div class="rec-name">Nothing at all</div>
              <div class="rec-artist">ESH</div>
            </div>
          </div>
          <div class="recomentation">
            <img src="images/santa.png" alt="" class="rec-image">
            <div class="rec-text">
              <div class="rec-name">Marry x</div>
              <div class="rec-artist">Julien Bam</div>
            </div>
          </div>
          <div class="recomentation">
            <img src="images/business.png" alt="" class="rec-image">
            <div class="rec-text">
              <div class="rec-name">Business</div>
              <div class="rec-artist">Tiësto</div>
            </div>
          </div>
        </div>
        <div id="last" class="last">
          <div class="last-item">
            <img src="" alt="" class="last-img">
            <div class="last-text">
              <div class="last-name"></div>
              <div class="last-artisr"></div>
            </div>
            <div class="last-time"></div>
            <img src="" alt="" class="last-play-img">
          </div>
          <div class="last-item">
            <img src="" alt="" class="last-img">
            <div class="last-text">
              <div class="last-name"></div>
              <div class="last-artisr"></div>
            </div>
            <div class="last-time"></div>
            <img src="" alt="" class="last-play-img">
          </div>
          <div class="last-item">
            <img src="" alt="" class="last-img">
            <div class="last-text">
              <div class="last-name"></div>
              <div class="last-artisr"></div>
            </div>
            <div class="last-time"></div>
            <img src="" alt="" class="last-play-img">
          </div>
          <div class="last-item">
            <img src="" alt="" class="last-img">
            <div class="last-text">
              <div class="last-name"></div>
              <div class="last-artisr"></div>
            </div>
            <div class="last-time"></div>
            <img src="" alt="" class="last-play-img">
          </div>
          <div class="last-item">
            <img src="" alt="" class="last-img">
            <div class="last-text">
              <div class="last-name"></div>
              <div class="last-artisr"></div>
            </div>
            <div class="last-time"></div>
            <img src="" alt="" class="last-play-img">
          </div>
          <div class="last-item">
            <img src="" alt="" class="last-img">
            <div class="last-text">
              <div class="last-name"></div>
              <div class="last-artisr"></div>
            </div>
            <div class="last-time"></div>
            <img src="" alt="" class="last-play-img">
          </div>
        </div>
      </div>


      <div id="player" class="player">
        <div class="song-info">
          <img onclick="window.location='miniplayer.html';" style="--bgsrc: url(../images/violett.png);" class="song-img"
            border="none">
          <div class="text-info">
            <div class="name">Violet</div>
            <div class="artist">Connor Price & Killa</div>
          </div>
        </div>
        <div class="controlls">
          <div class="progress-controlls">
            <img onclick="togglePlayButton(this.id, false);" src="images/svg/bold/backward.svg" id="back" class="back">
            <img onclick="togglePlayButton(this.id, true);" src="images/svg/bold/play.svg" id="pauseplay"
              class="pause-play">
            <img onclick="togglePlayButton(this.id, false);" src="images/svg/bold/forward.svg" id="for" class="for">
          </div>
          <div class="progressbar">
            <div class="progress-time">0:23</div>
            <div class="bar">
              <div style="width: 30%;" class="bar-filled"></div>
            </div>
            <div class="time-left">-1:53</div>
          </div>
        </div>
        <div class="sound-controlls">
          <img src="images/svg/bold/volume-mute.svg" class="less">
          <div class="sound-bar">
            <div style="width: 30%;" class="sound-bar-filled"></div>
          </div>
          <img src="images/svg/bold/volume-high.svg" class="more">
        </div>
      </div>
    </div>
  </div>
  <div id="contextmenu">
    <a href="#" class="cm-link">
      <img src="images/svg/linear/setting-2.svg" alt="" class="cm-icon">
      <div class="cm-text">Settings</div>
    </a>
    <a href="#" class="cm-link">
      <img src="images/svg/linear/user.svg" alt="" class="cm-icon">
      <div class="cm-text">User Settings</div>
    </a>
    <a href="#" class="cm-link">
      <img src="images/svg/linear/links.svg" alt="" class="cm-icon">
      <div class="cm-text">Links/Help</div>
    </a>
  </div>
  <div id="overlay">Minimum Size for desktop version is 1000px * 500px</div>
</template>

<script lang="js">
export default {
  ssr: false
}
</script>

<style>
@import '~/css/general.css';
@import '~/css/main.css';
@import '~/css/media.css';
@import '~/css/miniplayer.css';
@import '~/css/player.css';
@import '~/css/sidebar.css';
@import '~/css/titlebar.css';
</style>