<template>
  <div id="app">
    <NuxtLayout>
      <titlebar />
      <NuxtPage />
    </NuxtLayout>
  </div>
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
  overflow: hidden;
  background-color: transparent !important;
}
#app {
  background-color: var(--bg);
  height: 100vh;
  width: 100vw;
  border-radius: 6px;
}
</style>

<script>

</script>