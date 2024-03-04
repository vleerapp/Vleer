<template>
  <Navigation />
  <NuxtPage class="page" />
</template>


<style>
body {
  background-color: black;
  color: white !important;
  font-family: Inter;
  text-decoration: none !important;
  font-size: 16px;
}

* {
  font-family: Inter;
  text-decoration: none !important;
  font-size: 16px;
  padding: 0;
  margin: 0;
}

.page{
  position: absolute;
  height: calc(100vh - 30px);
  width: 100vw;
  top: 30px;
}
</style>