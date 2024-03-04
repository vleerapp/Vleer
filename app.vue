<template>
  <Navigation />
  <NuxtPage class="page" />
</template>


<style>
body {
  background-color: black;
  color: white;
  font-family: Inter;
  font-size: 16px;
}

* {
  margin: 0;
  padding: 0;
  font-family: Inter;
  box-sizing: border-box;
}

::-webkit-scrollbar {
  width: 3px;
  background-color: #000;
}

::-webkit-scrollbar-thumb {
  background: #6b6b6b;
}

::-webkit-scrollbar-thumb:window-inactive {
  background: #6b6b6b;
}

a{
  text-decoration: none !important;
}

.page{
  position: absolute;
  height: calc(100vh - 30px);
  width: 100vw;
  top: 30px;
  overflow-x: hidden;
  overflow-y: scroll;
}
</style>