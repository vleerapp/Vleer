<template>
  <div class="pages element">
    <p class="element-title">Pages</p>
    <NuxtLink class="link" to="/">
      <div class="svg-default">
        <IconsPagesHomeOutline />
      </div>
      <div class="svg-active">
        <IconsPagesHomeFill />
      </div>
      Home
    </NuxtLink>

    <NuxtLink class="link" to="/songs">
      <div class="svg-default">
        <IconsPagesSongsOutline />
      </div>
      <div class="svg-active">
        <IconsPagesSongsFill />
      </div>
      Songs
    </NuxtLink>

    <NuxtLink class="link" to="/search">
      <div class="svg-default">
        <IconsPagesSearchOutline />
      </div>
      <div class="svg-active">
        <IconsPagesSearchFill />
      </div>
      Search
    </NuxtLink>
    
    <NuxtLink class="link" to="/settings">
      <div class="svg-default">
        <IconsPagesSettingsOutline/>
      </div>
      <div class="svg-active">
        <IconsPagesSettingsFill/>
      </div>
      Settings
    </NuxtLink>
  </div>
</template>

<style lang="scss">
@import "~/assets/styles/components/pages.scss";
</style>
