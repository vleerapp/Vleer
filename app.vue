<template>
  <Titlebar />
  <Navigation />
  <Library />
  <Player />
  <div class="main">
    <p class="title">Main</p>
    <NuxtPage class="page" />
  </div>
</template>


<style lang="scss">
@font-face {
  font-family: 'DMMono';
  font-display: swap;
  src: url("/DMMono.woff2") format("woff2");

}

body {
  background-color: #121212;
  color: white !important;
  font-family: DMMono;
  text-decoration: none !important;
}

* {
  font-family: DMMono;
  text-decoration: none !important;
  padding: 0;
  margin: 0;
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

a {
  text-decoration: none !important;
}

.page {
  position: absolute;
  top: 30px;
  overflow-x: hidden;
  padding: 14px;
}

.main {
  position: absolute;
  top: 39px;
  left: 330px;
  width: calc(100vw - 380px);
  height: calc(100vh - 201px);
  padding: 16px;
  border: 1px solid #535353;

  .title {
    font-size: 14px;
    position: absolute;
    top: -10px;
    left: 4px;
    background-color: #121212;
    padding-inline: 3px;
    color: #535353;
  }
}

.main:hover {
  border: 1px solid #A058FF;
}

.main:hover .title {
  color: #A058FF;
}
</style>