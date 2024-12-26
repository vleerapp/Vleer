<template>
  <div class="default-layout">
    <Navbar/>
    <Library />
    <slot />
    <Player />
  </div>
</template>

<style lang="scss">
@use '~/assets/styles/layouts/default_layout';
</style>
