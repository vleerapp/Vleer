<template>
  <sidebar />
  <div id="content" class="content">
    <slot />
  </div>
  <div class="bg"></div>
</template>

<style scoped>
body {
  position: relative;
}

.sidebar {
  position: fixed;
}

.content {
  position: absolute;
  left: 372px;
  transition: .2s;
  top: 110px;
}

@media screen and (max-width: 1200px) {
  .content {
    left: 152px;
  }
}
</style>