<template>
  <sidebar />
  <div id="content" class="content">
    <slot />
  </div>
  <div class="bg"></div>
  <player />
</template>

<style scoped>
body {
  position: relative;
}

.sidebar {
  position: fixed;
}

.bg {
  width: 100%;
  height: 200px;
  position: fixed;
  background-color: var(--bg);
  bottom: 0;
  left: 350px;
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