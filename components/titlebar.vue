<template>
  <div data-tauri-drag-region class="titlebar">
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" class="titlebar-button" id="titlebar-minimize"
      xmlns="http://www.w3.org/2000/svg">
      <path d="M2 9V6.5C2 4.01 4.01 2 6.5 2H9" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round"
        stroke-linejoin="round" />
      <path d="M15 2H17.5C19.99 2 22 4.01 22 6.5V9" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round"
        stroke-linejoin="round" />
      <path d="M22 16V17.5C22 19.99 19.99 22 17.5 22H16" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round"
        stroke-linejoin="round" />
      <path d="M9 22H6.5C4.01 22 2 19.99 2 17.5V15" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round"
        stroke-linejoin="round" />
    </svg>

    <svg id="titlebar-minimize" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M2 9V6.5C2 4.01 4.01 2 6.5 2H9" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round"
        stroke-linejoin="round" />
      <path d="M15 2H17.5C19.99 2 22 4.01 22 6.5V9" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round"
        stroke-linejoin="round" />
      <path d="M22 16V17.5C22 19.99 19.99 22 17.5 22H16" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round"
        stroke-linejoin="round" />
      <path d="M9 22H6.5C4.01 22 2 19.99 2 17.5V15" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round"
        stroke-linejoin="round" />
    </svg>


    <img src="../assets/svg/linear/minimize.svg" stroke="white" class="titlebar-button" id="titlebar-minimize" />
    <img src="../assets/svg/linear/maximize.svg" class="titlebar-button" id="titlebar-maximize" />
    <img src="../assets/svg/linear/close.svg" class="titlebar-button" id="titlebar-close" />
  </div>
</template>

<style>
@import '~/css/titlebar.css';
</style>