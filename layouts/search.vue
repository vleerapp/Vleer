<template>
  <sidebar />
  <div class="content">
    <slot />
  </div>
  <div class="bg"></div>
</template>

<style>
body {
  position: relative;
}
.sidebar {
  position: fixed;
}
.content {
  position: absolute;
  top: 45px;
  left: 372px;
  transition: .2s;
}

@media screen and (max-width: 1200px) {
  .content {
    left: 152px;
  }
}
</style>