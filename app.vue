<template>
  <div>
    <NuxtLayout>
      <titlebar />
      <NuxtPage />
    </NuxtLayout>
  </div>
  <div id="app"></div>
</template>

<style>
:root {
  /*=======COLORS===========*/
  --bg: #16151a;
  --sidebar: #19181d;
  --e: #1c1b21;
  --dtx: #85868b;
  --tx: #ffffff;
  --t: #e06c75;
  --search: #1e1d23;
  --border: #26252b;
}

body {
  scroll-behavior: smooth;
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
  border-radius: 6px;
  z-index: -1;
}
</style>

<script>

</script>