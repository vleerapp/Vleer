<template>
  <div class="default-layout">
    <div class="content">
      <div class="sidebar">
        <Navigation />
        <Library />
      </div>
      <div class="main element">
        <p class="element-title">hey</p>
        <slot />
      </div>
    </div>
    <Player />
  </div>
</template>

<style lang="scss">
@use '~/assets/styles/default_layout';
</style>