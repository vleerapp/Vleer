<template>
  <div class="player element">
    <p class="element-title">Player</p>
    <div class="top">
      <div class="info">
        <img :src="coverUrl || '/cover.png'" class="cover"></img>
        <div class="h">
          <div class="title">{{ truncate(currentSong.title) }}</div>
          <div class="artist">{{ truncate(currentSong.artist) }}</div>
        </div>
      </div>
      <div class="controls">
        <IconsShuffle />
        <IconsRewind />
        <IconsPlay v-if="paused" @click="play" />
        <IconsPause v-if="!paused" @click="pause" />
        <IconsSkip />
        <IconsRepeat @click="toggleLoop" :class="{ 'active': looping }" />
      </div>
      <div class="right-controls">
        <IconsVolumeLoud @click="mute" v-if="volume > 50" />
        <IconsVolumeMid @click="mute" v-else-if="volume > 0" />
        <IconsVolumeMute @click="mute" v-else />

        <input @input="setVolume" v-model="volume" step="1" min="0" max="100" type="range" name="" id="">

        <div class="volume-text">{{ volume }}%</div>
      </div>
    </div>
    <div class="bottom">
      <input type="range" class="progress" v-model="progress" @input="skipTo" min="0" max="100" step=".1" />
      <div class="progress-indicator" :style="{ width: progress + '%' }"></div>
      <div class="numbers">{{ time }} / {{ audio.duration > 0
        ? new Date(audio.duration * 1000).toISOString().substr(14, 5)
        : "00:00" }}</div>
    </div>
  </div>
</template>

<script lang="ts" setup>
const { $music, $settings } = useNuxtApp();

const paused = ref(true)
const looping = ref(false)
const time = ref("00:00")
const progress = ref($music.getAudio().currentTime)
const audio = ref($music.getAudio())
const volume = ref($settings.getVolume());
setVolume()
const coverUrl = ref('/cover.png');

audio.value.addEventListener('pause', () => {
  paused.value = true
})

audio.value.addEventListener('play', () => {
  paused.value = false
})

audio.value.addEventListener('ended', () => {
  if (looping.value) {
    $music.play()
  }
})

audio.value.addEventListener('timeupdate', () => {
  time.value = audio.value.currentTime > 0 ? new Date(audio.value.currentTime * 1000).toISOString().substr(14, 5) : "00:00";
  progress.value = (audio.value.currentTime / audio.value.duration) * 100;
})

function play() {
  $music.play();
}

function pause() {
  $music.pause();
}

function skipTo() {
  audio.value.currentTime = (progress.value / 100) * audio.value.duration;
}

function toggleLoop() {
  looping.value = !looping.value
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
    } catch (error) {
      console.error('Error fetching cover URL:', error);
      coverUrl.value = '/cover.png';
    }
  } else if (!newSong.id) {
    coverUrl.value = '/cover.png';
  }
}, { immediate: true });

function mute() {
  volume.value = 0
  $music.setVolume(volume.value);
  $settings.setVolume(volume.value)
}

function setVolume() {
  $music.setVolume(volume.value);
  $settings.setVolume(volume.value)
}

function truncate(text: string, length: number = 30) {
  return text.length > length ? text.substring(0, length - 3) + '...' : text;
}
</script>

<style lang="scss">
@import '~/assets/styles/components/player.scss';
</style>