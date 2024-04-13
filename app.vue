<template>
  <Titlebar />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script lang="ts" setup>
const { $music } = useNuxtApp();

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