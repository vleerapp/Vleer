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
  overflow-x: hidden;
  background-color: transparent !important;
  position: relative;
}
::-webkit-scrollbar {
  display: none;
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