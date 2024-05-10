<template>
  <Titlebar />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script lang="ts" setup>
import Database from "@tauri-apps/plugin-sql";

window.addEventListener('error', (e) => {
  if (e.target instanceof HTMLAudioElement) {
    const mediaError = e.target.error;
    if (mediaError) {
      console.error("Global error handler: Error with audio element:", mediaError);
      console.error("Global error handler: MediaError code:", mediaError.code);
      switch(mediaError.code) {
        case mediaError.MEDIA_ERR_ABORTED:
          console.error("Global error handler: The fetching process for the media resource was aborted by the user agent at the user's request.");
          break;
        case mediaError.MEDIA_ERR_NETWORK:
          console.error("Global error handler: A network error caused the user agent to stop fetching the media resource, after the resource was established to be usable.");
          break;
        case mediaError.MEDIA_ERR_DECODE:
          console.error("Global error handler: An error of some description occurred while decoding the media resource, after the resource was established to be usable.");
          break;
        case mediaError.MEDIA_ERR_SRC_NOT_SUPPORTED:
          console.error("Global error handler: The media resource indicated by the src attribute or assigned media provider object was not suitable.");
          break;
        default:
          console.error("Global error handler: An unknown error occurred.");
          break;
      }
    }
  }
}, true);

const { $music, $settings } = useNuxtApp();

await $music.init();

const isTextInputFocused = ref(false);

onMounted(async () => {
  document.addEventListener('keydown', handleKeyDown);
  document.addEventListener('focusin', updateFocus);
  document.addEventListener('focusout', updateFocus);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeyDown);
  document.removeEventListener('focusin', updateFocus);
  document.removeEventListener('focusout', updateFocus);
});

function handleKeyDown(event: KeyboardEvent) {
  if (event.code === 'Space' && !isTextInputFocused.value) {
    $music.playPause();
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