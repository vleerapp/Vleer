<template>
  <div class="player element">
    <p class="element-title">Player</p>
    <div class="top">
      <div class="info">
        <img :src="coverUrl || '/cover.png'" class="cover"></img>
        <div class="h">
          <div class="title">{{ currentSong.title }}</div>
          <div class="artist">{{ currentSong.artist }}</div>
        </div>
      </div>
      <div class="controls">
        <IconsShuffle />
        <IconsRewind />
        <IconsPlay v-if="paused" @click="play" />
        <IconsPause v-if="!paused" @click="pause" />
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
    <!-- <div class="bottom">
      <div class="progress" @click="seekTo($event)" :style="{ width: progress + '%' }" style="cursor: pointer;">
        <div class="indicator"></div>
      </div>
      <div class="numbers">
        {{ time }} / {{ duration }}
      </div>
    </div> -->
  </div>
</template>

<script lang="ts" setup>
import type { Song } from '~/types/music';

const { $music } = useNuxtApp();

const volume = ref($music.getAudio().volume * 100);
const paused = ref($music.getAudio().paused);
const coverUrl = ref('/cover.png');

function play() {
  $music.play();
  paused.value = false;
}

function pause() {
  $music.pause();
  paused.value = true;
}

const currentSong = computed(() => {
  return $music.getCurrentSong() || {
    title: 'No song playing',
    artist: 'Unknown',
    cover: '/cover.png'
  };
});

watch(currentSong, async (newSong, oldSong) => {
  if (newSong.id && newSong.id !== (oldSong ? oldSong.id : null)) {
    try {
      coverUrl.value = await $music.getCoverURLFromID(newSong.id);
      paused.value = $music.getAudio().paused
    } catch (error) {
      console.error('Error fetching cover URL:', error);
      coverUrl.value = '/cover.png';
    }
  } else if (!newSong.id) {
    coverUrl.value = '/cover.png';
  }
}, { immediate: true });

function setVolume() {
  $music.setVolume(volume.value);
}
</script>

<style lang="scss">
@import '~/assets/styles/components/player.scss';
</style>