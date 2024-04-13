<template>
  <Titlebar />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script lang="ts" setup>
import { register } from '@tauri-apps/plugin-global-shortcut';
const { $music } = useNuxtApp();

await $music.init();

await register('KeyS', () => {
  console.log('Shortcut triggered');
});
</script>

<style lang="scss">
@use '~/assets/styles/global';
</style>