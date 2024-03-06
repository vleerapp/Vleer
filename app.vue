<template>
  <Titlebar />
  <Navigation/>
  <NuxtPage class="page" />
</template>


<style>
@font-face {
  font-family: 'DMMono';
  font-style: normal;
  font-weight: 100 900;
  font-display: swap;
  src: url("/DMMono.woff2") format("woff2");
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
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