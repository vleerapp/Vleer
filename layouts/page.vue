<template>
  <sidebar />
  <div class="content">
    <search />
    <slot />
  </div>
</template>

<style>
.content {
  position: absolute;
  top: 40px;
  left: 372px;
}
</style>