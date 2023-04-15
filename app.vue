<template>
  <titlebar />
  <NuxtPage />
  <div id="app"></div>
  <div id="overlay">
    <img src="/svg/linear/download.svg" width="100" height="100" />
  </div>
</template>

<style>
:root {
  /*=======COLORS===========*/
  --bg: #16151a;
  --sidebar: #19181d;
  --accent: #7b00ff;
  --accenth: #6c00e0;
  --element: #1C1B21;
  --elementh: #232229;
  --tx: #ffffff;
  --dtx: #85868b;
  --search: #1e1d23;
  --border: #26252b;
}

@font-face {
  font-family: Inter;
  src: url("/Inter.woff2") format("woff2");
  font-display: swap;
}

* {
  scroll-behavior: smooth;
}

body {
  overflow-x: hidden;
  background-color: transparent !important;
  position: relative;
}

::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
  margin: 3px;
  margin-top: 45px;
}

::-webkit-scrollbar-thumb {
  background: var(--sidebar);
  border-radius: 12px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--search);
}

#app {
  background-color: var(--bg);
  position: fixed;
  height: 100vh;
  width: 100vw;
  border-radius: 10px;
  z-index: -1;
}

#overlay {
  pointer-events: none;
  opacity: 0;
  transition: 0.5s;
  position: fixed;
  margin: 10px;
  height: calc(100vh - 20px);
  width: calc(100vw - 20px);
  border-radius: 5px;
  z-index: 10;
  border: 5px solid var(--accent);
  display: grid;
  place-items: center;
}
</style>

<script setup>
import { appWindow } from "@tauri-apps/api/window";

appWindow.onResized(async () => {
  var isma = await appWindow.isMaximized();
  if (isma) {
    document.getElementById("app").style.borderRadius = "0";
  } else {
    document.getElementById("app").style.borderRadius = "6px";
  }
});

document.body.addEventListener("scroll", () => {
  if (document.body.scrollTop < 800) {
    document.getElementById("gotoTop").style.opacity = "0";
    document.getElementById("gotoTop").style.pointerEvents = "none";
  } else {
    document.getElementById("gotoTop").style.opacity = "1";
    document.getElementById("gotoTop").style.pointerEvents = "all";
  }
});

appWindow.onFileDropEvent((event) => {
  var overlay = document.getElementById("overlay");
  if (event.payload.type === "hover") {
    overlay.style.opacity = "1";
  } else if (event.payload.type === "drop") {
    overlay.style.opacity = "0";
    console.log("User dropped", event.payload.paths);
  } else {
    overlay.style.opacity = "0";
  }
});
</script>
