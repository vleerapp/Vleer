<template>
  <div class="player element">
    <p class="element-title">Player</p>
    <div class="top">
      <div class="info">
        <img :src="cover || '/cover.png'" class="cover"></img>
        <div class="h">
          <div class="title">{{ title }}</div>
          <div class="artist">{{ artist }}</div>
        </div>
      </div>
      <div class="controls">
        <IconsShuffle />
        <IconsRewind />
        <IconsPlay v-if="!playing" @click="play()" />
        <IconsPause v-if="playing" @click="pause()" />
        <IconsSkip />
        <IconsRepeat />
      </div>
      <div class="right-controls">
        <IconsVolumeLoud v-if="volume > 50" />
        <IconsVolumeMid v-else-if="volume > 0" />
        <IconsVolumeMute v-else />

        <input @input="setVolume" v-model="volume" step="1" min="0" max="100" type="range" name="" id="">
      </div>
    </div>
    <div class="bottom">
      <div class="progress" @click="seekTo($event)" :style="{ width: progress + '%' }" style="cursor: pointer;">
        <div class="indicator"></div>
      </div>
      <div class="numbers">
        {{ time }} / {{ duration }}
      </div>
    </div>
  </div>
</template>

<script>
import Player from '@/lib/Player.ts';

export default {
  data() {
    return {
      player: Player.getInstance(),
      title: 'Title',
      artist: 'Artist',
      cover: "",
      volume: 100,
      time: "0:00",
      duration: "0:00",
      progress: 0,
      playing: false
    };
  },
  methods: {
    seekTo(event) {
      const progressContainer = this.$el.querySelector('.progress');
      const clickX = event.pageX - progressContainer.offsetLeft;
      const width = progressContainer.offsetWidth;
      const clickProgress = (clickX / width) * 100;
      console.log(clickProgress)
      this.player.seek(clickProgress * this.player.getDuration() / 100);
    },
    play() {
      this.player.play();
    },
    pause() {
      this.player.pause();
    },
    setVolume() {
      this.player.setVolume(this.volume / 100);
    },
    toggleMute() {
      this.player.toggleMute();
    },
    async getTitle(id) {
      this.title = await this.player.getTitle(id)
    },
    async getArtist(id) {
      this.artist = await this.player.getArtist(id)
    },
    async getCover(id) {
      this.cover = await this.player.getCover(id)
    },
  },
  mounted() {
    this.player.audio.addEventListener('timeupdate', () => {
      const time = this.player.getCurrentTime();
      this.time = time > 0 ? new Date(time * 1000).toISOString().substr(14, 5) : '0:00';
      this.progress = (this.player.getCurrentTime() / this.player.getDuration()) * 100;
    });
    this.player.audio.addEventListener('loadedmetadata', () => {
      const duration = this.player.getDuration();
      this.duration = duration > 0 ? new Date(duration * 1000).toISOString().substr(14, 5) : '0:00';
      this.getTitle(this.player.currentSongId);
      this.getArtist(this.player.currentSongId);
      this.getCover(this.player.currentSongId);
    });
    this.player.audio.addEventListener('play', () => {
      this.playing = true;
    });
    this.player.audio.addEventListener('pause', () => {
      this.playing = false;
    });
  }
}
</script>

<style lang="scss">
@import '~/assets/styles/components/player.scss';
</style>