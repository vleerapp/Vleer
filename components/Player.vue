<template>
  <div class="player element">
    <p class="element-title">Player</p>
    <div class="top">
      <div class="info">
        <img :src="coverUrl || '/cover.png'" class="cover"></img>
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
        <IconsPlay v-if="paused" @click="play" />
        <IconsPause v-if="!paused" @click="pause" />
        <IconsSkip @click="skip" />
        <IconsRepeat @click="toggleLoop" :class="{ 'active': looping }" />
      </div>
      <div class="right-controls">
        <IconsVolumeLoud @click="mute" v-if="volume > 50" />
        <IconsVolumeMid @click="mute" v-else-if="volume > 0" />
        <IconsVolumeMute @click="mute" v-else />

        <div class="bar">
          <input class="range" @input="setVolume" v-model="volume" step="1" min="0" max="100" type="range" name=""
            id="">
          <div class="volume-indicator" :style="{ width: volume + '%' }"></div>
        </div>

        <div class="volume-text">{{ volume }}%</div>
      </div>
    </div>
    <div class="bottom">
      <input type="range" class="progress" v-model="progress" @input="skipTo" min="0" max="100" step=".1" />
      <div class="progress-indicator" :style="{ width: progress + '%' }"></div>
      <div class="numbers">{{ time }} / {{ duration }}</div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/core";
import { Howl } from 'howler';
import type { Song } from '~/types/types';

const { $music, $settings } = useNuxtApp();

const paused = ref(true)
const looping = ref(false)
const time = ref("00:00")
const duration = ref("00:00")
const progress = ref(0)
const volume = ref($settings.getVolume());
const coverUrl = ref('/cover.png');

const currentSong = ref<Partial<Song> | null>(null);

watch($music.getCurrentSong, async (songPromise) => {
  const song = await songPromise;
  currentSong.value = song ? {
    id: song.id,
    title: song.title,
    artist: song.artist,
    cover: song.cover,
  } : null;
}, { immediate: true });

watch(() => $music.howl, (newHowl: Howl | null) => {
  if (newHowl) {
    newHowl.on('play', onPlay);
    newHowl.on('pause', onPause);
    newHowl.on('end', onEnd);
    newHowl.on('load', onLoad);
    newHowl.on('loaderror', onLoadError);
    newHowl.on('playerror', onPlayError);
  }
}, { immediate: true });

function onPlay() {
  paused.value = false;
  updateDiscordActivity();
}

function onPause() {
  paused.value = true;
  clearDiscordActivity();
}

function onEnd() {
  if (looping.value) {
    $music.play();
  } else {
    clearDiscordActivity();
  }
}

function onLoad() {
  updateDuration();
}

function onLoadError() {
  console.error("Error loading audio");
}

function onPlayError() {
  console.error("Error playing audio");
}

function updateDuration() {
  if ($music.howl) {
    duration.value = formatTime($music.howl.duration());
  }
}

function formatTime(secs: number) {
  const minutes = Math.floor(secs / 60) || 0;
  const seconds = Math.floor(secs - minutes * 60) || 0;
  return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
}

async function updateDiscordActivity() {
  try {
    if (!currentSong.value) return;

    let thumbnail;
    try {
      if (currentSong.value?.id) {
        const response = await fetch(`https://api.wireway.ch/wave/thumbnail/${encodeURIComponent(currentSong.value.id)}`);
        const data = await response.json();
        thumbnail = data.items[0].thumbnail;
      }
    } catch (error) {
      thumbnail = "https://discussions.apple.com/content/attachment/592590040"
      console.error("Failed to fetch song thumbnail:", error);
    }

    await invoke("update_activity", {
      state: "by " + currentSong.value.artist,
      details: currentSong.value.title,
      largeImage: thumbnail,
      largeImageText: currentSong.value.title,
      youtube_url: "https://youtube.com/watch?v=" + currentSong.value.id
    });
  } catch (error) {
    console.error("Failed to update Discord activity:", error);
  }
}

async function clearDiscordActivity() {
  try {
    await invoke("clear_activity")
  } catch (error) {
    console.error("Failed to clear Discord activity:", error);
  }
}

function play() {
  $music.playPause();
}

function pause() {
  $music.playPause();
}

function skip() {
  $music.skip();
}

function rewind() {
  $music.rewind();
}

function skipTo() {
  if ($music.howl) {
    $music.howl.seek(($music.howl.duration() * progress.value) / 100);
  }
}

function toggleLoop() {
  looping.value = !looping.value
  if ($music.howl) {
    $music.howl.loop(looping.value);
  }
}

watch(currentSong, async (newSong, oldSong) => {
  if (newSong && newSong.id && newSong.id !== (oldSong ? oldSong.id : null)) {
    try {
      coverUrl.value = await $music.getCoverURLFromID(newSong.id);
    } catch (error) {
      console.error('Error fetching cover URL:', error);
      coverUrl.value = '/cover.png';
    }
  } else if (newSong && !newSong.id) {
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

function truncate(text: string | undefined, length: number = 30) {
  if (!text) return '';
  return text.length > length ? text.substring(0, length - 3) + '...' : text;
}

const updateInterval = setInterval(() => {
  if ($music.howl && $music.howl.playing()) {
    const seek = $music.howl.seek() || 0;
    time.value = formatTime(seek);
    progress.value = (seek / $music.howl.duration()) * 100 || 0;
  }
}, 1000);

onUnmounted(() => {
  clearInterval(updateInterval);
});
</script>

<style lang="scss">
@import '~/assets/styles/components/player.scss';
</style>