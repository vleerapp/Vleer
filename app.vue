<template>
  <Titlebar />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script lang="ts" setup>
const { $music, $settings } = useNuxtApp();

const eq = $settings.getEq()
await $music.init();
</script>

<style lang="scss">
@use '~/assets/styles/global';
</style>