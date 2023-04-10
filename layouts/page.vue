<template>
  <sidebar />
  <div class="content">
    <search />
    <slot />
  </div>
</template>

<style>
body {
  background-color: transparent;
}
.content {
  position: absolute;
  top: 40px;
  left: 372px;
  transition: .2s;
}

@media screen and (max-width: 1200px) {
  .content {
    left: 152px;
  }
}
</style>