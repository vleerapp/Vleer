<template>
  <Titlebar />
  <Navigation/>
  <NuxtPage class="page" />
</template>


<style>
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
</style>