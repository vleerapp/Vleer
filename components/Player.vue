<template>
  <div class="player element">
    <p class="element-title">Player</p>
    <div class="top">
      <div class="info">
        <img src="/cover.png" class="cover"></img>
        <div class="h">
          <div class="title">{{ title }}</div>
          <div class="artist">{{ artist }}</div>
        </div>
      </div>
      <div class="controls">
        <svg width="32px" height="32px" viewBox="0 0 32 32" class="icon shuffle">
          <g id="shuffle">
            <path d="M-1930 -2100L-1930 -2100L-1930 -2068L-1962 -2068L-1962 -2100L-1930 -2100Z" id="shuffle" fill="none"
              stroke="none" />
            <path
              d="M7.97754 21.9517L10 21.9517L18 13L20 13L20 15L24 11.5L20 8L20 11L17 11L9.23621 19.5L7.97754 19.5L7.97754 21.9517Z"
              id="Vector" fill="currentColor" fill-rule="evenodd" stroke="none" clip-path="url(#clip_1)" />
            <path d="M8 10L10.0225 10L12.9521 12.9519L11.5 14.5L9.5 12.4517L8 12.4517L8 10Z" id="Vector-3"
              fill="currentColor" fill-rule="evenodd" stroke="none" clip-path="url(#clip_1)" />
            <path
              d="M18.0225 18.9517L20.0225 18.9517L20.0225 16.9517L24.0225 20.4517L20.0225 23.9517L20.0225 20.9517L17.0225 20.9517L15 19.0181L16.498 17.5L18.0225 18.9517Z"
              id="Vector-2" fill="currentColor" fill-rule="evenodd" stroke="none" clip-path="url(#clip_1)" />
          </g>
        </svg>
        <svg width="32px" height="32px" viewBox="0 0 32 32" class="icon rewind">
          <g id="backwards">
            <path d="M-1962 -2068L-1962 -2100L-1930 -2100L-1930 -2068L-1962 -2068Z" id="backwards" fill="none"
              stroke="none" transform="matrix(-1 0 -0 -1 32 32)" />
            <path
              d="M12 22.9227L12 9.07733L22.872 16L12 22.9227ZM13.3333 20.4667L20.384 16L13.3333 11.5333L13.3333 20.4667Z"
              id="Shape" fill="currentColor" fill-rule="evenodd" stroke="none" clip-path="url(#clip_1)"
              transform="matrix(-1 0 -0 -1 32 32)" />
          </g>
        </svg>
        <svg v-if="!playing" width="32px" height="32px" viewBox="0 0 32 32" class="icon play">
          <g id="play">
            <path d="M-1930 -2100L-1930 -2100L-1930 -2068L-1962 -2068L-1962 -2100L-1930 -2100Z" id="play" fill="none"
              stroke="none" />
            <path d="M12 22.9227L12 9.07739L22.872 16.0001L12 22.9227Z" id="Shape" fill="currentColor" stroke="none"
              clip-path="url(#clip_1)" />
          </g>
        </svg>
        <svg v-if="playing" width="32px" height="32px" viewBox="0 0 32 32" class="icon pause">
          <g id="pause">
            <path d="M-1930 -2100L-1930 -2100L-1930 -2068L-1962 -2068L-1962 -2100L-1930 -2100Z" id="pause" fill="none"
              stroke="none" />
            <path d="M10 9L15 9L15 23L10 23L10 9Z" id="Rectangle" fill="currentColor" fill-rule="evenodd" stroke="none"
              clip-path="url(#clip_1)" />
            <path d="M16 9L21 9L21 23L16 23L16 9Z" id="Rectangle" fill="currentColor" fill-rule="evenodd" stroke="none"
              clip-path="url(#clip_1)" />
          </g>
        </svg>
        <svg width="32px" height="32px" viewBox="0 0 32 32" class="icon skip">
          <g id="skip">
            <path d="M-1930 -2100L-1930 -2100L-1930 -2068L-1962 -2068L-1962 -2100L-1930 -2100Z" id="skip" fill="none"
              stroke="none" />
            <path
              d="M12 22.9227L12 9.07733L22.872 16L12 22.9227ZM13.3333 20.4667L20.384 16L13.3333 11.5333L13.3333 20.4667Z"
              id="Shape" fill="currentColor" fill-rule="evenodd" stroke="none" clip-path="url(#clip_1)" />
          </g>
        </svg>
        <svg width="32px" height="32px" viewBox="0 0 32 32" class="icon repeat">
          <g id="repeat">
            <path d="M-1962 -2068L-1962 -2100L-1930 -2100L-1930 -2068L-1962 -2068Z" id="refresh" fill="none"
              stroke="none" transform="matrix(0 -1 1 0 0 32)" />
            <path
              d="M20.2364 11.7625C19.1146 10.6337 17.5882 9.99925 15.9962 10C12.6792 10 10 12.685 10 16C10 19.315 12.6792 22 15.9962 22C18.7955 22 21.1295 20.0875 21.7974 17.5L20.2364 17.5C19.6028 19.2976 17.9033 20.5001 15.9962 20.5C13.5122 20.5 11.4934 18.4825 11.4934 16C11.4934 13.5175 13.5122 11.5 15.9962 11.5C17.242 11.5 18.3527 12.0175 19.1632 12.835L16.7467 15.25L22 15.25L22 10L20.2364 11.7625Z"
              id="Shape" fill="currentColor" stroke="none" clip-path="url(#clip_1)" transform="matrix(0 -1 1 0 0 32)" />
          </g>
        </svg>
      </div>
      <div class="right-controls">
        <svg v-if="volume > 50" width="32px" height="32px" viewBox="0 0 32 32" class="icon loud">
          <g id="Loud">
            <path d="M-2796 -2106L-2796 -2106L-2796 -2074L-2828 -2074L-2828 -2106L-2796 -2106Z" id="Loud" fill="none"
              stroke="none" />
            <path
              d="M8.06399 14.648C7.97405 15.2385 7.97934 15.8415 8.07721 16.437C8.22535 17.349 8.7994 18.1455 9.64058 18.6104C11.9446 19.8842 17.4016 22.9019 17.4016 22.9019C17.6423 23.0325 17.9361 23.0325 18.1768 22.9019C18.4149 22.7712 18.563 22.53 18.563 22.2687L18.563 8.73339C18.563 8.47208 18.4149 8.22834 18.1768 8.09768C17.9361 7.96703 17.6423 7.96702 17.4016 8.10019C17.4016 8.10019 11.9446 11.1153 9.64058 12.3892C8.7994 12.854 8.22535 13.653 8.07721 14.5651L8.06399 14.648L8.06399 14.648ZM19.8318 10.0499C19.8318 10.0223 19.845 9.99464 19.8715 9.97956C19.8953 9.96197 19.9268 9.95945 19.9533 9.9695C22.3181 10.8037 24.0005 12.967 24.0005 15.4997C24.0005 18.0349 22.3181 20.1957 19.9533 21.0325C19.9268 21.0425 19.8953 21.0375 19.8715 21.0224C19.8476 21.0048 19.8318 20.9797 19.8318 20.952L19.8318 19.5852C19.8318 19.5274 19.8661 19.4721 19.9217 19.447C21.4268 18.7108 22.4584 17.2183 22.4584 15.4997C22.4584 13.7811 21.4268 12.2911 19.9217 11.5549C19.8661 11.5273 19.8318 11.4745 19.8318 11.4142L19.8318 10.0499L19.8318 10.0499ZM9.60075 14.7913C9.67746 14.3164 9.97914 13.8993 10.4156 13.6581L17.0179 10.0073L17.0179 20.7989C17.0179 20.8391 16.9969 20.8768 16.9598 20.8969C16.9228 20.917 16.8805 20.917 16.8435 20.8969C15.0765 19.9195 12.0001 18.221 10.4156 17.3441C9.97914 17.1029 9.67746 16.6858 9.60075 16.2109C9.52404 15.7411 9.52404 15.2612 9.60075 14.7913L9.60075 14.7913ZM19.8318 12.8818C19.8318 12.8466 19.8529 12.8139 19.8847 12.7988C19.9191 12.7837 19.9587 12.7862 19.9878 12.8089C20.8316 13.432 21.3765 14.4069 21.3765 15.4998C21.3765 16.5953 20.8316 17.5677 19.9878 18.1934C19.9587 18.2135 19.9191 18.216 19.8847 18.2009C19.8529 18.1858 19.8318 18.1532 19.8318 18.1205L19.8318 12.8818L19.8318 12.8818Z"
              id="Volume" fill="currentColor" fill-rule="evenodd" stroke="none" clip-path="url(#clip_1)" />
          </g>
        </svg>
        <svg v-else-if="volume > 0" width="32px" height="32px" viewBox="0 0 32 32" class="icon mid">
          <g id="Mid">
            <path d="M-2796 -2106L-2796 -2106L-2796 -2074L-2828 -2074L-2828 -2106L-2796 -2106Z" id="Mid" fill="none"
              stroke="none" />
            <path
              d="M8.06399 14.648C7.97405 15.2385 7.97934 15.8415 8.07721 16.437C8.22535 17.349 8.7994 18.1455 9.64058 18.6104C11.9446 19.8842 17.4016 22.9019 17.4016 22.9019C17.6423 23.0325 17.9361 23.0325 18.1768 22.9019C18.4149 22.7712 18.563 22.53 18.563 22.2687L18.563 8.73339C18.563 8.47208 18.4149 8.22834 18.1768 8.09768C17.9361 7.96703 17.6423 7.96702 17.4016 8.10019C17.4016 8.10019 11.9446 11.1153 9.64058 12.3892C8.7994 12.854 8.22535 13.653 8.07721 14.5651L8.06399 14.648L8.06399 14.648ZM9.60075 14.7913C9.67746 14.3164 9.97914 13.8993 10.4156 13.6581L17.0179 10.0073L17.0179 20.7989C17.0179 20.8391 16.9969 20.8768 16.9598 20.8969C16.9228 20.917 16.8805 20.917 16.8435 20.8969C15.0765 19.9195 12.0001 18.221 10.4156 17.3441C9.97914 17.1029 9.67746 16.6858 9.60075 16.2109C9.52404 15.7411 9.52404 15.2612 9.60075 14.7913L9.60075 14.7913ZM19.8318 12.8818C19.8318 12.8466 19.8529 12.8139 19.8847 12.7988C19.9191 12.7837 19.9587 12.7862 19.9878 12.8089C20.8316 13.432 21.3765 14.4069 21.3765 15.4998C21.3765 16.5953 20.8316 17.5677 19.9878 18.1934C19.9587 18.2135 19.9191 18.216 19.8847 18.2009C19.8529 18.1858 19.8318 18.1532 19.8318 18.1205L19.8318 12.8818L19.8318 12.8818Z"
              id="Volume" fill="currentColor" fill-rule="evenodd" stroke="none" clip-path="url(#clip_1)" />
          </g>
        </svg>
        <svg v-else width="32px" height="32px" viewBox="0 0 32 32" class="icon mute">
          <g id="Mute">
            <path d="M-2796 -2106L-2796 -2106L-2796 -2074L-2828 -2074L-2828 -2106L-2796 -2106Z" id="Mute" fill="none"
              stroke="none" />
            <path
              d="M8.06399 14.648C7.97405 15.2385 7.97934 15.8415 8.07721 16.437C8.22535 17.349 8.7994 18.1455 9.64058 18.6104C11.9446 19.8842 17.4016 22.9019 17.4016 22.9019C17.6423 23.0325 17.9361 23.0325 18.1768 22.9019C18.4149 22.7712 18.563 22.53 18.563 22.2687L18.563 8.73339C18.563 8.47208 18.4149 8.22834 18.1768 8.09768C17.9361 7.96703 17.6423 7.96702 17.4016 8.10019C17.4016 8.10019 11.9446 11.1153 9.64058 12.3892C8.7994 12.854 8.22535 13.653 8.07721 14.5651L8.06399 14.648L8.06399 14.648ZM9.60075 14.7913C9.67746 14.3164 9.97914 13.8993 10.4156 13.6581L17.0179 10.0073L17.0179 20.7989C17.0179 20.8391 16.9969 20.8768 16.9598 20.8969C16.9228 20.917 16.8805 20.917 16.8435 20.8969C15.0765 19.9195 12.0001 18.221 10.4156 17.3441C9.97914 17.1029 9.67746 16.6858 9.60075 16.2109C9.52404 15.7411 9.52404 15.2612 9.60075 14.7913L9.60075 14.7913Z"
              id="Volume" fill="currentColor" fill-rule="evenodd" stroke="none" clip-path="url(#clip_1)" />
            <path
              d="M20.1629 13.1642L20.1642 13.1629Q20.1966 13.1305 20.239 13.1129Q20.2813 13.0954 20.3271 13.0954Q20.3729 13.0954 20.4153 13.1129Q20.4576 13.1305 20.49 13.1629L24.8371 17.51Q24.8695 17.5424 24.8871 17.5847Q24.9046 17.6271 24.9046 17.6729Q24.9046 17.7187 24.887 17.761Q24.8695 17.8034 24.8371 17.8358L24.8358 17.8371Q24.8034 17.8695 24.761 17.8871Q24.7187 17.9046 24.6729 17.9046Q24.6271 17.9046 24.5847 17.8871Q24.5424 17.8695 24.51 17.8371L20.1629 13.49Q20.1305 13.4576 20.1129 13.4153Q20.0954 13.3729 20.0954 13.3271Q20.0954 13.2813 20.1129 13.239Q20.1305 13.1966 20.1629 13.1642Z"
              id="Rectangle" fill="none" fill-rule="evenodd" stroke="currentColor" stroke-width="1"
              clip-path="url(#clip_1)" />
            <path
              d="M24.8358 13.1629L24.8371 13.1642Q24.8695 13.1966 24.8871 13.239Q24.9046 13.2813 24.9046 13.3271Q24.9046 13.3729 24.8871 13.4153Q24.8695 13.4576 24.8371 13.49L20.49 17.8371Q20.4576 17.8695 20.4153 17.8871Q20.3729 17.9046 20.3271 17.9046Q20.2813 17.9046 20.239 17.8871Q20.1966 17.8695 20.1642 17.8371L20.1629 17.8358Q20.1305 17.8034 20.1129 17.761Q20.0954 17.7187 20.0954 17.6729Q20.0954 17.6271 20.1129 17.5847Q20.1305 17.5424 20.1629 17.51L24.51 13.1629Q24.5424 13.1305 24.5847 13.1129Q24.6271 13.0954 24.6729 13.0954Q24.7187 13.0954 24.761 13.1129Q24.8034 13.1305 24.8358 13.1629L24.8358 13.1629Z"
              id="Rectangle" fill="none" fill-rule="evenodd" stroke="currentColor" stroke-width="1"
              clip-path="url(#clip_1)" />
          </g>
        </svg>
      </div>
    </div>
    <div class="bottom">
      <div class="progress" @click="seekTo($event)" :style="{ width: progress + '%' }" style="cursor: pointer;">
        <div class="indicator"></div>
      </div>
      <div class="numbers">
        {{ currentTimeFormatted }} / {{ durationFormatted }}
      </div>
    </div>
  </div>
</template>

<script>
import Player from '@/lib/Player.ts';

export default {
  data() {
    return {
      player: new Player(),
      title: 'Song Title',
      artist: 'Artist Name',
    };
  },
  computed: {
    playing() {
      return !this.player.isStopped;
    },
    volume() {
      return this.player.lastVolume * 100;
    },
    progress() {
      return this.player.getDuration() > 0 ? this.player.getProgress() : 0;
    },
    currentTimeFormatted() {
      const time = this.player.getCurrentTime();
      return this.player.getDuration() > 0 ? new Date(time * 1000).toISOString().substr(14, 5) : '0:00';
    },
    durationFormatted() {
      const duration = this.player.getDuration();
      return duration > 0 ? new Date(duration * 1000).toISOString().substr(14, 5) : '0:00';
    }
  },
  methods: {
    seekTo(event) {
      const progressContainer = this.$el.querySelector('.progress');
      const clickX = event.pageX - progressContainer.offsetLeft;
      const width = progressContainer.offsetWidth;
      const clickProgress = (clickX / width) * 100;
      this.player.seek(clickProgress * this.player.getDuration() / 100);
    },
    play() {
      this.player.playPause();
    },
    pause() {
      this.player.playPause();
    },
    setVolume(volume) {
      this.player.setVolume(volume / 100);
    },
    toggleMute() {
      this.player.toggleMute();
    }
  }
};
</script>

<style lang="scss">
@import '~/assets/styles/components/player.scss';
</style>