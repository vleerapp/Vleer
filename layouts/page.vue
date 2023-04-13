<template>
  <sidebar />
  <div id="content" class="content">
    <search />
    <slot />
  </div>
  <div class="bg"></div>
</template>

<style>
.bg {
  background-color: red;
}

.content {
  position: absolute;
  top: 40px;
  left: 372px;
  transition: 0.2s;
  width: calc(100% - 450px);
}

@media screen and (max-width: 1200px) {
  .content {
    left: 152px;
    width: calc(100% - 250px);
  }
}
</style>
