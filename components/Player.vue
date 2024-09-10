<template>
  <div class="player element">
    <p class="element-title">Player</p>
    <div class="top">
      <div class="info">
        <img :src="currentSong?.cover || '/cover.png'" class="cover" alt="Album cover">
        <div class="h" v-if="currentSong">
          <div class="title">{{ truncate(currentSong.title) }}</div>
          <div class="artist">{{ truncate(currentSong.artist) }}</div>
        </div>
        <div class="h" v-else>
          <div class="title">No song playing</div>
          <div class="artist">Unknown</div>
        </div>
      </div>
      <div class="controls">
        <IconsShuffle />
        <IconsRewind @click="rewind" />
        <IconsPlay v-if="paused" @click="playPause" />
        <IconsPause v-if="!paused" @click="playPause" />
        <IconsSkip @click="skip" />
        <IconsRepeat @click="toggleLoop" :class="{ 'active': looping }" />
      </div>
      <div class="right-controls">
        <IconsVolumeLoud @click="mute" v-if="volume > 50" />
        <IconsVolumeMid @click="mute" v-else-if="volume > 0" />
        <IconsVolumeMute @click="mute" v-else />

        <div class="bar">
          <input class="range" @input="setVolume" v-model="volume" step="1" min="0" max="100" type="range">
          <div class="volume-indicator" :style="{ width: volume + '%' }"></div>
        </div>

        <div class="volume-text">{{ volume }}%</div>
      </div>
    </div>
    <div class="bottom">
      <input
        type="range"
        class="progress"
        :value="progress"
        @input="skipTo"
        min="0"
        max="100"
        step=".1"
      />
      <div class="progress-indicator" :style="{ width: progress + '%' }"></div>
      <div class="numbers">{{ formatTime(currentTime) }} / {{ formatTime(duration) }}</div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, watch, onMounted } from 'vue';
import type { Song } from '~/types/types';

const { $music, $player, $settings } = useNuxtApp();

const currentSong = ref<Song | null>(null);
const currentTime = ref(0);
const duration = ref(0);
const looping = ref(false);
const muted = ref(false);
const paused = ref(true);
const progress = ref(0);
const volume = ref(50);

onMounted(async () => {
  looping.value = $player.looping.value;
  muted.value = $player.muted.value;
  volume.value = $player.volume.value;

  const newSong = await $settings.getCurrentSong();
  if (newSong) {
    const song = await $music.getSong(newSong.id);
    if (song) {
      await $player.loadSong(song);
      currentSong.value = song;
    }
  }

  watch(() => $player.currentSong.value, (newSong) => {
    currentSong.value = newSong;
  });
  
  watch(() => $player.duration.value, (newDuration) => {
    duration.value = newDuration;
  });

  watch(() => $player.paused.value, (newPaused) => {
    paused.value = newPaused;
  });

  watch(() => $player.progress.value, (newProgress) => {
    progress.value = newProgress;
    currentTime.value = (newProgress / 100) * duration.value;
  });
  
  watch(() => $player.time.value, (newTime) => {
    currentTime.value = newTime;
    progress.value = (newTime / duration.value) * 100;
  });

  watch(() => $player.looping.value, (newLoop) => {
    looping.value = newLoop;
  });

  watch(() => $player.muted.value, (newMuted) => {
    muted.value = newMuted;
  });

  watch(() => $player.volume.value, (newVolume) => {
    volume.value = newVolume;
  });
});

const playPause = () => $player.playPause();
const skip = () => $player.skip();
const rewind = () => $player.rewind();
const skipTo = (e: Event) => {
  const target = e.target as HTMLInputElement;
  const newProgress = parseFloat(target.value);
  $player.skipTo(newProgress);
  progress.value = newProgress;
  currentTime.value = (newProgress / 100) * duration.value;
};
const setVolume = (e: Event) => {
  const target = e.target as HTMLInputElement;
  $player.setVolume(parseInt(target.value));
};
const mute = () => $player.mute();
const toggleLoop = () => $player.toggleLoop();

const truncate = (text: string | undefined, length: number = 30) => {
  if (!text) return '';
  return text.length > length ? text.substring(0, length - 3) + '...' : text;
};

const formatTime = (seconds: number): string => {
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = Math.floor(seconds % 60);
  return `${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`;
};
</script>

<style lang="scss">
@import '~/assets/styles/components/player.scss';
</style>