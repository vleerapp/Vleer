<template>
  <Titlebar />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script lang="ts" setup>
import { register, unregister, isRegistered } from '@tauri-apps/plugin-global-shortcut';
import { initDb } from '~/services/db';
import initializeSettings from '~/plugins/settings';

const { $player, $settings } = useNuxtApp();

onMounted(async () => {
  await initDb();
  await initializeSettings(useNuxtApp())

  document.addEventListener('keydown', handleKeyDown);
  document.addEventListener('focusin', updateFocus);
  document.addEventListener('focusout', updateFocus);

  if (await isRegistered("MediaPlayPause")) {
    await unregister("MediaPlayPause")
  }

  await register('MediaPlayPause', (event) => {
    if (event.state === "Pressed") {
      $player.playPause()
    }
  });

  if (await isRegistered("MediaTrackNext")) {
    await unregister("MediaTrackNext")
  }

  await register('MediaTrackNext', (event) => {
    if (event.state === "Pressed") {
      $player.skip()
    }
  });

  if (await isRegistered("MediaTrackPrevious")) {
    await unregister("MediaTrackPrevious")
  }

  await register('MediaTrackPrevious', (event) => {
    if (event.state === "Pressed") {
      $player.rewind();
    }
  });
});

onUnmounted(async () => {
  document.removeEventListener('keydown', handleKeyDown);
  document.removeEventListener('focusin', updateFocus);
  document.removeEventListener('focusout', updateFocus);

  if (await isRegistered("MediaPlayPause")) {
    await unregister("MediaPlayPause")
  }

  if (await isRegistered("MediaTrackNext")) {
    await unregister("MediaTrackNext")
  }

  if (await isRegistered("MediaTrackPrevious")) {
    await unregister("MediaTrackPrevious")
  }
});

const isTextInputFocused = ref(false);

function handleKeyDown(event: KeyboardEvent) {
  if (event.code === 'Space' && !isTextInputFocused.value) {
    $player.playPause();
    event.preventDefault();
  }
}

function updateFocus() {
  const activeElement = document.activeElement;
  isTextInputFocused.value = activeElement instanceof HTMLInputElement ||
    activeElement instanceof HTMLTextAreaElement;
}
</script>

<style lang="scss">
@use '~/assets/styles/global';
</style>