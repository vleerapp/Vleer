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
  --accent: #7B00FF;
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
</style>

<script>
export default {
  mounted() {
    document.body.addEventListener("scroll", () => {
      if (document.body.scrollTop < 800) {
        document.getElementById("gotoTop").style.opacity = "0";
        document.getElementById("gotoTop").style.pointerEvents = "none";
      } else {
        document.getElementById("gotoTop").style.opacity = "1";
        document.getElementById("gotoTop").style.pointerEvents = "all";

      }
    });
  },
};

</script>
