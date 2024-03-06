<template>
  <div class="nav">
    <p class="title">Pages</p>
    <NuxtLink class="link" to="/">
      <div class="svg-default">
        <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns:xlink="http://www.w3.org/1999/xlink"
          xmlns="http://www.w3.org/2000/svg">
          <g id="outline">
            <path d="M0 0L16 0L16 16L0 16L0 0Z" id="Rectangle" fill="none" fill-rule="evenodd" stroke="none" />
            <path
              d="M3.5 12.5557L5.75 12.5557L5.75 8.22233L10.25 8.22233L10.25 12.5557L12.5 12.5557L12.5 6.05566L8 2.80566L3.5 6.05566L3.5 12.5557ZM2 14L2 5.33333L8 1L14 5.33333L14 14L8.75 14L8.75 9.66667L7.25 9.66667L7.25 14L2 14Z"
              id="Shape" fill-rule="evenodd" stroke="none" />
          </g>
        </svg>
      </div>
      <div class="svg-active">
        <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns:xlink="http://www.w3.org/1999/xlink"
          xmlns="http://www.w3.org/2000/svg">
          <g id="fill">
            <path d="M0 0L16 0L16 16L0 16L0 0Z" id="Rectangle-2" fill="none" fill-rule="evenodd" stroke="none" />
            <path
              d="M2 14L2 5.33342L8.00725 1L14 5.33342L14 14L9.50616 14L9.50616 8.94475L6.50034 8.94475L6.50034 14L2 14Z"
              id="Shape" stroke="none" />
          </g>
        </svg>
      </div>
      Home
    </NuxtLink>
    <NuxtLink class="link" to="/search">
      <div class="svg-default">
        <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns:xlink="http://www.w3.org/1999/xlink"
          xmlns="http://www.w3.org/2000/svg">
          <g id="Outline">
            <path d="M0 0L16 0L16 16L0 16L0 0Z" id="Shape-2" fill="none" fill-rule="evenodd" stroke="none" />
            <path
              d="M6.99998 1.3335C4.51783 1.33371 2.32472 2.94929 1.58867 5.31979C0.852627 7.6903 1.74498 10.2639 3.79048 11.67C5.83599 13.076 8.55847 12.9872 10.508 11.4508L12.9426 13.8855C13.2042 14.1381 13.62 14.1345 13.8772 13.8774C14.1343 13.6202 14.138 13.2044 13.8853 12.9428L11.4506 10.5082C12.7946 8.80318 13.0464 6.4802 12.0988 4.52689C11.1512 2.57359 9.17099 1.33335 6.99998 1.3335M2.66663 7.00033C2.66663 4.60709 4.60673 2.66699 6.99996 2.66699C9.39319 2.66699 11.3333 4.60709 11.3333 7.00033C11.3333 9.39356 9.39319 11.3337 6.99996 11.3337C4.60673 11.3337 2.66663 9.39356 2.66663 7.00033"
              id="Shape" fill-rule="evenodd" stroke="none" />
          </g>
        </svg>
      </div>
      <div class="svg-active">
        <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns:xlink="http://www.w3.org/1999/xlink"
          xmlns="http://www.w3.org/2000/svg">
          <g id="Fill">
            <path d="M0 0L16 0L16 16L0 16L0 0Z" id="Shape" fill="none" fill-rule="evenodd" stroke="none" />
            <path
              d="M6.99998 1.33325C9.17098 1.33311 11.1512 2.57334 12.0988 4.52665C13.0463 6.47995 12.7946 8.80293 11.4506 10.5079L13.8853 12.9426C14.138 13.2042 14.1343 13.62 13.8772 13.8771C13.62 14.1343 13.2042 14.1379 12.9426 13.8853L10.508 11.4506C8.55847 12.987 5.83598 13.0758 3.79048 11.6697C1.74498 10.2637 0.852628 7.69005 1.58867 5.31955C2.32472 2.94904 4.51783 1.33346 6.99998 1.33325M6.99996 2.6665C4.60673 2.6665 2.66663 4.6066 2.66663 6.99984C2.66663 9.39307 4.60673 11.3332 6.99996 11.3332C9.39319 11.3332 11.3333 9.39307 11.3333 6.99984C11.3333 4.6066 9.39319 2.6665 6.99996 2.6665M6.99992 3.33325C9.02496 3.33325 10.6666 4.97487 10.6666 6.99992C10.6666 9.02496 9.02496 10.6666 6.99992 10.6666C4.97487 10.6666 3.33325 9.02496 3.33325 6.99992C3.33325 4.97487 4.97487 3.33325 6.99992 3.33325"
              id="Shape" fill-rule="evenodd" stroke="none" />
          </g>
        </svg>
      </div>
      Search
    </NuxtLink>
  </div>
</template>

<style lang="scss">
@import '~/css/navigation.scss';
</style>