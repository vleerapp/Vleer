<template>
  <sidebar />
  <div id="content" class="content">
    <search />
    <slot />
  </div>
  <player />
</template>

<style>

.sidebar {
  position: fixed;
}

.content {
  position: absolute;
  top: 100px;
  left: 372px;
  transition: 0.2s;
  width: calc(100% - 450px);
  padding-bottom: 100px;
}

@media screen and (max-width: 1200px) {
  .content {
    left: 152px;
    width: calc(100% - 250px);
  }
}
</style>
