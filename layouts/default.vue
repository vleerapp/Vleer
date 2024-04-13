<template>
  <div class="default-layout">
    <div class="content">
      <div class="sidebar">
        <Pages />
        <Library />
      </div>
      <slot />
    </div>
    <Player />
  </div>
</template>

<style lang="scss">
@use '~/assets/styles/layouts/default_layout';
</style>
